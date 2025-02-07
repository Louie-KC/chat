use std::str::FromStr;

use const_format::formatcp;
use serde_json::json;

use common::{
    AccountPasswordChange,
    AccountRequest,
    ChatMessage,
    ChatRoomManageUser,
    ChatRoomName,
    LoginResponse,
    LoginTokenInfo,
    UserAssociationUpdate,
    UserInfo
};

use actix_web::{
    get,
    post,
    put,
    web::{
        Data, Json, Path, Query, ServiceConfig
    },
    http::header,
    HttpRequest,
    HttpResponse,
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

use crate::{
    database::{
        DatabaseService,
        DatabaseServiceError,
    },
    models::UserSearchParam,
};

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
        // Account management
        .service(register)
        .service(login)
        .service(change_password)
        .service(clear_token)
        .service(get_all_tokens)
        .service(clear_all_tokens)
        // Chat room management
        .service(get_room_list)
        .service(create_chat_room)
        .service(change_room_name)
        .service(get_room_member_names)
        .service(manage_room_members)
        // Chat interaction
        .service(chat_get_messages)
        .service(chat_send_message)
        // User interaction
        .service(user_search_global)
        .service(user_association)
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

// Account management

#[post("/account/register")]
async fn register(
    db_service: Data<DatabaseService>,
    argon2: Data<Argon2<'_>>,
    body: Json<AccountRequest>,
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

#[post("/account/login")]
async fn login(
    db_service: Data<DatabaseService>,
    argon2: Data<Argon2<'_>>,
    req: HttpRequest,
    body: Json<AccountRequest>,
    // req: HttpRequest
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
    
    // Get user-agent header
    let headers = req.headers();
    let user_agent = match headers.get(header::USER_AGENT) {
        Some(data) => data.to_str().unwrap_or("Unknown client"),
        None => "Unknown client"
    };

    // Generate and store token before sending back to client
    // In the very unlikely chance a UUID V4 clash occurs, re-try
    let mut token = Uuid::new_v4();
    let mut token_set_result = Err(DatabaseServiceError::KeyAlreadyExists);
    while let Err(DatabaseServiceError::KeyAlreadyExists) = token_set_result {
        token = Uuid::new_v4();
        token_set_result = db_service.user_set_token(&db_user_data.id, &token, user_agent).await;
    }

    match token_set_result {
        Ok(()) => HttpResponse::Ok().json(LoginResponse {
            user_id: db_user_data.id,
            token: token.to_string()
        }),
        Err(_) => HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[post("/account/change-password")]
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

    // Identify requesting user, then get current User record
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
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

#[post("/account/logout")]
pub async fn clear_token(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth
) -> HttpResponse {
    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Checked unwrap as function called above ensures the token is valid
    let token = Uuid::from_str(bearer.token()).unwrap();

    match db_service.user_remove_token(&user_id, &token).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/account/tokens")]
pub async fn get_all_tokens(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    req: HttpRequest
) -> HttpResponse {
    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Checked unwrap, valid per above match
    let token = Uuid::from_str(bearer.token()).unwrap();

    // Get user-agent header
    let headers = req.headers();
    let user_agent = match headers.get(header::USER_AGENT) {
        Some(data) => data.to_str().unwrap_or("Unknown client"),
        None => "Unknown client"
    };

    // Update user-agent for current token
    if let Err(_) = db_service.user_set_token(&user_id, &token, user_agent).await {
        return HttpResponse::InternalServerError().reason("1").finish()
    }

    match db_service.user_get_associated_tokens(&user_id, &token).await {
        Ok(db_info) => {
            let info = db_info.iter().map(Into::<LoginTokenInfo>::into).collect::<Vec<_>>();
            HttpResponse::Ok().json(info)
        },
        Err(_) => HttpResponse::InternalServerError().reason("2").finish(),
    }
}

#[post("/account/clear-tokens")]
pub async fn clear_all_tokens(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth
) -> HttpResponse {

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    match db_service.user_clear_tokens_by_id(&user_id).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("2").finish(),
    }
}

// Chat room management

#[get("/chat/rooms")]
async fn get_room_list(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth
) -> HttpResponse {
    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    match db_service.chat_room_list_for_user(&user_id).await {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    }
}

#[post("/chat/create-room")]
async fn create_chat_room(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    body: Json<ChatRoomName>
) -> HttpResponse {
    // Input validation
    if body.room_name.is_empty() {
        return HttpResponse::BadRequest().reason("Empty room_name value").finish();
    }
    if body.room_name.len() > MAX_USERNAME_LEN {
        return HttpResponse::BadRequest().reason("room_name value longer than 64 chars").finish();
    }

    if body.room_name.chars().any(|c| !c.is_ascii_alphanumeric() && c.ne(&' ')) {
        return HttpResponse::BadRequest().reason(NON_ALLOWED_CHARACTER_REASON).finish();
    }

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Create chat room
    let room_id = match db_service.chat_room_create(&body.room_name).await {
        Ok(room_id) => room_id,
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Add requesting user to the chat room
    match db_service.chat_room_add_user(&room_id, &user_id).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[put("/chat/{room_id}/change-name")]
async fn change_room_name(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    path: Path<u64>,
    body: Json<ChatRoomName>
) -> HttpResponse {
    let room_id = path.into_inner();
    
    // Input validation
    if body.room_name.is_empty() {
        return HttpResponse::BadRequest().reason("Empty room_name value").finish();
    }
    if body.room_name.len() > MAX_USERNAME_LEN {
        return HttpResponse::BadRequest().reason("room_name value longer than 64 chars").finish();
    }

    if body.room_name.chars().any(|c| !c.is_ascii_alphanumeric() && c.ne(&' ')) {
        return HttpResponse::BadRequest().reason(NON_ALLOWED_CHARACTER_REASON).finish();
    }

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Verify that user is in the room_id
    let room_users = match db_service.chat_room_get_users(&room_id).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    if let None = room_users.iter().find(|m| m.user_id == user_id) {
        return HttpResponse::Unauthorized().reason("Not part of the room").finish()
    }

    match db_service.chat_room_change_name(&room_id, &body.room_name).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("3").finish(),
    }
}

#[get("/chat/{room_id}/members")]
async fn get_room_member_names(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    path: Path<u64>
) -> HttpResponse {
    let room_id = path.into_inner();

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    let members = match db_service.chat_room_get_users(&room_id).await {
        Ok(members) => members,
        Err(DatabaseServiceError::NoResult) => Vec::new(),
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Check if requestng user is in the room
    if !members.iter().map(|m| m.user_id).any(|id| id == user_id) {
        return HttpResponse::Unauthorized().reason("User is not part of the room").finish()
    }

    let members_list = members.iter()
        .map(|m| UserInfo { id: m.user_id, username: m.username.clone() })
        .collect::<Vec<UserInfo>>();

    HttpResponse::Ok().json(members_list)
}

#[post("/chat/{room_id}/manage-user")]
async fn manage_room_members(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    path: Path<u64>,
    body: Json<ChatRoomManageUser>
) -> HttpResponse {
    let room_id = path.into_inner();

    // Identify requesting user
    let requester_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Get room members
    let room_members = match db_service.chat_room_get_users(&room_id).await {
        Ok(members) => members,
        Err(DatabaseServiceError::NoResult) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().reason("2").finish(),
    };

    // Ensure requester is in room
    if let None = room_members.iter().find(|m| m.user_id == requester_id) {
        return HttpResponse::Unauthorized().reason("Must be part of the room being managed").finish()
    }

    let user_present = room_members.iter()
        .find(|m| m.user_id == body.user_id)
        .is_some();

    match body.action {
        common::ChatRoomManageUserAction::AddUser if user_present => {
            HttpResponse::Ok().finish()
        },
        common::ChatRoomManageUserAction::AddUser => {
            match db_service.chat_room_add_user(&room_id, &body.user_id).await {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().reason("3").finish()
            }
        }
        common::ChatRoomManageUserAction::RemoveUser if user_present => {
            match db_service.chat_room_remove_user(&room_id, &body.user_id).await {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().reason("4").finish(),
            }
        },
        common::ChatRoomManageUserAction::RemoveUser => {
            HttpResponse::BadRequest().reason("User being removed is not part of the room").finish()
        }
    }
}


// Chat interaction
#[get("/chat/{room_id}/{offset}/{limit}")]
async fn chat_get_messages(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    path: Path<(u64, u64, u64)>
) -> HttpResponse {
    let (room_id, offset, limit) = path.into_inner();

    if limit == 0 {
        return HttpResponse::BadRequest().reason("limit parameter was 0").finish()
    }

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Check if requestng user is in the room
    let members = match db_service.chat_room_get_users(&room_id).await {
        Ok(members) => members,
        Err(DatabaseServiceError::NoResult) => Vec::new(),
        Err(_) => return HttpResponse::InternalServerError().reason("1").finish(),
    };

    if !members.iter().map(|m| m.user_id).any(|id| id == user_id) {
        return HttpResponse::Unauthorized().reason("User is not part of the room").finish()
    }

    // Retrieve messages to be returned
    match db_service.chat_room_read_messages(&room_id, &offset, &limit).await {
        Ok(msg_window) => HttpResponse::Ok().json(msg_window),
        Err(_) => HttpResponse::InternalServerError().reason("2").finish(),
    }
}

#[post("/chat")]
async fn chat_send_message(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    body: Json<ChatMessage>
) -> HttpResponse {
    // Disallow optional fields being populated
    if body.id.is_some() || body.sender_id.is_some() || body.time_sent.is_some() {
        return HttpResponse::BadRequest().reason("id, sender_id, or time_sent fields have values").finish()
    }

    // Identify requesting user
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Check if requestng user is in the room
    let members = match db_service.chat_room_get_users(&body.room_id).await {
        Ok(members) => members,
        Err(DatabaseServiceError::NoResult) => Vec::new(),
        Err(_) => return HttpResponse::InternalServerError().reason("1").finish(),
    };

    if !members.iter().map(|m| m.user_id).any(|id| id == user_id) {
        return HttpResponse::Unauthorized().reason("User is not part of the room").finish()
    }

    // Record new message
    match db_service.chat_room_send_message(&user_id, &body).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().reason("2").finish(),
    }
}

// User interaction

#[get("/users")]
async fn user_search_global(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    query: Query<UserSearchParam>
) -> HttpResponse {
    // Search must contain some text
    if query.username.is_empty() {
        return HttpResponse::BadRequest().reason("username query cannot be empty").finish()
    }
    
    // User must be logged in to search. They may be blocked
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    match db_service.user_search_global(&user_id, &query.username).await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(_) => HttpResponse::InternalServerError().reason("1").finish(),
    }
}

#[post("/users")]
async fn user_association(
    db_service: Data<DatabaseService>,
    bearer: BearerAuth,
    body: Json<UserAssociationUpdate>
) -> HttpResponse {
    // Find requesting user id
    let user_id = match token_to_user_id(&db_service, bearer.token()).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    use common::UserAssociationType::*;
    let update_result = match body.association_type {
        Friend => db_service.user_association_set_friend(&user_id, &body.other_user_id).await,
        Block  => db_service.user_association_set_block(&user_id, &body.other_user_id).await,
        Remove => db_service.user_association_delete(&user_id, &body.other_user_id).await,
    };

    match update_result {
        Ok(()) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(DatabaseServiceError::NoResult) => HttpResponse::Ok().json(json!({"status": "no change"})),
        Err(_) => HttpResponse::InternalServerError().reason("1").finish()
    }
}


// Util

/// Find the user id that the provided `bearer_token` maps to.
/// 
/// If the provided `bearer_token` is in an incorrect format, or does not map
/// to a user id, then the appropriate HttpResponse is returned.
/// * bad token format - HTTP 400 Bad Request
/// * no mapped user - HTTP 401 Unauthorized
async fn token_to_user_id(db_service: &DatabaseService, bearer_token: &str) -> Result<u64, HttpResponse> {
    let token = match Uuid::from_str(bearer_token) {
        Ok(uuid) => uuid,
        Err(_) => return Err(HttpResponse::BadRequest().reason(BAD_TOKEN_FORMAT_REASON).finish()),
    };
    match db_service.user_id_from_token(&token).await {
        Ok(user_id) => Ok(user_id),
        Err(DatabaseServiceError::NoResult) => Err(HttpResponse::Unauthorized().finish()),
        Err(_) => Err(HttpResponse::InternalServerError().reason("map").finish()),
    }
}