use const_format::formatcp;
use serde_json::json;

use common::AccountRequest;

use actix_web::{
    get,
    post,
    web::{
        Data, Json, ServiceConfig
    },
    HttpResponse
};

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

pub fn config(config: &mut ServiceConfig) -> () {
    config.service(actix_web::web::scope("")
        .service(health)
        .service(register)
        .service(login)
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
    let db_user_data = match db_service.user_get(&body.username).await {
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
