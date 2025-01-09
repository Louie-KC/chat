use std::str::FromStr;

use const_format::formatcp;
use serde_json::json;

use common::{AccountPasswordChange, AccountRequest};

use actix_web::{
    get,
    post,
    web::{
        Data, Json, ServiceConfig
    },
    HttpResponse
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};
use uuid::Uuid;

use crate::{database::DatabaseServiceError, DatabaseService};

const MIN_USERNAME_LEN: usize = 4;
const MAX_USERNAME_LEN: usize = 64;
const MIN_PASSWORD_LEN: usize = 8;
const MAX_PASSWORD_LEN: usize = 64;

const BAD_USERNAME_REASON: &str = formatcp!("Username must be between {MIN_USERNAME_LEN} and {MAX_USERNAME_LEN} in length");
const BAD_PASSWORD_REASON: &str = formatcp!("Password must be between {MIN_PASSWORD_LEN} and {MAX_PASSWORD_LEN} in length");
const NON_ALLOWED_CHARACTER_REASON: &str = "A field contains dis-allowed characters. Alphanumeric only";
const BAD_TOKEN_FORMAT_REASON: &str = "Invalid bearer token format";

pub fn config(config: &mut ServiceConfig) -> () {
    config.service(actix_web::web::scope("")
        .service(health)
        .service(register)
        .service(login)
        .service(change_password)
        .service(clear_tokens)
    );
}

#[get("/health")]
async fn health(db_service: Data<DatabaseService>) -> HttpResponse {
    let db_connected = db_service.health_check().await.is_ok();

    let status = match db_connected {
        true  => "success",
        false => "error"
    };
    HttpResponse::Ok().json(json!({"status": status}))
}

#[post("/register")]
async fn register(
    db_service: Data<DatabaseService>,
    argon2: Data<Argon2<'_>>,
    body: Json<AccountRequest>
) -> HttpResponse {
    // Input validation
    if MIN_USERNAME_LEN > body.username.len() || body.username.len() > MAX_USERNAME_LEN {
        return HttpResponse::BadRequest().reason(BAD_USERNAME_REASON).finish()
    }
    if MIN_PASSWORD_LEN > body.password.len() || body.password.len() > MAX_PASSWORD_LEN {
        return HttpResponse::BadRequest().reason(BAD_PASSWORD_REASON).finish()
    }

    let username_invalid = body.username.chars().any(|c| !c.is_ascii_alphanumeric());
    let password_invalid = body.password.chars().any(|c| !c.is_ascii_alphanumeric());
    if username_invalid || password_invalid {
        return HttpResponse::BadRequest().reason(NON_ALLOWED_CHARACTER_REASON).finish();
    }

    // Check if username is already taken
    match db_service.user_exists(&body.username).await {
        Ok(false) => {}, // Do nothing
        Ok(true)  => return HttpResponse::BadRequest().reason("Username is already taken").finish(),
        Err(_)    => return HttpResponse::InternalServerError().reason("1").finish(),
    };

    // Password hashing
    let salt = SaltString::generate(&mut OsRng);
    let hash = match argon2.hash_password(body.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    log::warn!("hash len: {}", hash.len());
    // DB Create operation
    match db_service.user_register(&body.username, hash).await {
        Ok(_)  => HttpResponse::Ok().finish(),
        Err(_) => return HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[post("/login")]
async fn login(
    db_service: Data<DatabaseService>,
    argon2: Data<Argon2<'_>>,
    body: Json<AccountRequest>
) -> HttpResponse {
    // Input validation
    if MIN_USERNAME_LEN > body.username.len() || body.username.len() > MAX_USERNAME_LEN {
        return HttpResponse::BadRequest().reason(BAD_USERNAME_REASON).finish()
    }
    if MIN_PASSWORD_LEN > body.password.len() || body.password.len() > MAX_PASSWORD_LEN {
        return HttpResponse::BadRequest().reason(BAD_PASSWORD_REASON).finish()
    }

    // Check if username is already taken
    let username_invalid = body.username.chars().any(|c| !c.is_ascii_alphanumeric());
    let password_invalid = body.password.chars().any(|c| !c.is_ascii_alphanumeric());
    if username_invalid || password_invalid {
        return HttpResponse::BadRequest().reason(NON_ALLOWED_CHARACTER_REASON).finish();
    }

    // Retrieve User data for input username (if exists).
    let db_user_data = match db_service.user_get_by_username(&body.username).await {
        Ok(user) => user,
        Err(DatabaseServiceError::NoResult) => return HttpResponse::BadRequest().reason("Username does not exist").finish(),
        Err(_) => return HttpResponse::InternalServerError().reason("1").finish()
    };

    let stored_hash = match PasswordHash::new(&db_user_data.password_hash) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Verify input password
    if let Err(_) = argon2.verify_password(body.password.as_bytes(), &stored_hash) { 
        return HttpResponse::BadRequest().reason("Incorrect password").finish()
    };

    std::mem::drop(stored_hash);

    // Generate and store token before sending back to client
    // In the very unlikely chance a UUID V4 clash occurs, re-try
    let mut token = Uuid::new_v4();
    let mut token_set_result = Err(DatabaseServiceError::KeyAlreadyExists);
    while let Err(DatabaseServiceError::KeyAlreadyExists) = token_set_result {
        token = Uuid::new_v4();
        token_set_result = db_service.user_set_token(&db_user_data.id, &token).await;
    }

    match token_set_result {
        Ok(()) => HttpResponse::Ok().json(json!({"token": token.to_string()})),
        Err(_) => HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[post("/change-password")]
pub async fn change_password(
    db_service: Data<DatabaseService>,
    argon2: Data<Argon2<'_>>,
    bearer: BearerAuth,
    body: Json<AccountPasswordChange>
) -> HttpResponse {
    // Input validation
    if MIN_PASSWORD_LEN > body.new_password.len() || body.new_password.len() > MAX_PASSWORD_LEN {
        return HttpResponse::BadRequest().reason(BAD_PASSWORD_REASON).finish()
    }
    // Character check
    let old_password_invalid = body.old_password.chars().any(|c| !c.is_ascii_alphanumeric());
    let new_password_invalid = body.new_password.chars().any(|c| !c.is_ascii_alphanumeric());
    if old_password_invalid || new_password_invalid {
        return HttpResponse::BadRequest().reason(NON_ALLOWED_CHARACTER_REASON).finish();
    }

    // Ensure passwords are different
    if body.old_password.eq(&body.new_password) {
        return HttpResponse::BadRequest().reason("New and old passwords are identical").finish()
    }

    let token = match Uuid::from_str(bearer.token()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().reason(BAD_TOKEN_FORMAT_REASON).finish(),
    };

    // Retrieve user_id from token, then current User record
    let user_id = match db_service.user_id_from_token(&token).await {
        Ok(id) => id,
        Err(DatabaseServiceError::NoResult) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().reason("1").finish(),
    };

    let db_user_data = match db_service.user_get_by_id(&user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Check old_password is correct
    let stored_hash = match PasswordHash::new(&db_user_data.password_hash) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    if let Err(_) = argon2.verify_password(body.old_password.as_bytes(), &stored_hash) { 
        return HttpResponse::BadRequest().reason("Incorrect old password").finish()
    };
    std::mem::drop(stored_hash);

    // Generate hash for new_password
    let salt = SaltString::generate(&mut OsRng);
    let new_hash = match argon2.hash_password(body.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Update stored password hash
    match db_service.user_update_password_hash(&user_id, new_hash).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[post("/clear-tokens")]
pub async fn clear_tokens(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth
) -> HttpResponse {
    // bearer check
    let token = match Uuid::from_str(bearer.token()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().reason(BAD_TOKEN_FORMAT_REASON).finish(),
    };

    // Find the user_id associated with the token, if any
    let user_id = match db_service.user_id_from_token(&token).await {
        Ok(id) => id,
        Err(DatabaseServiceError::NoResult) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().reason("1").finish(),
    };

    match db_service.user_clear_tokens_by_id(&user_id).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("2").finish(),
    }
}
