use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

use crate::{
    models::{
        message::*,
        authenticate::{Login, LoginToken},
        account::Account
    },
    database::database::Database
};

trait StringExtension {
    fn invalid_username(&self) -> bool;
    fn invalid_password(&self) -> bool;
}

impl StringExtension for String {
    fn invalid_username(&self) -> bool {
        for ch in self.chars() {
            if !ch.is_ascii_alphanumeric() && ch.ne(&'_') {
                return true;
            }
        }
        false
    }

    fn invalid_password(&self) -> bool {
        for ch in self.chars() {
            if !ch.is_ascii_graphic() {
                return true
            }
        }
        false
    }
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "Server is online"}))
}

#[post("/register")]
pub async fn register(db: web::Data<Database>, details: web::Json<Account>) -> HttpResponse {
    // username and password validity checks
    if details.username.invalid_username() {
        let reason = "Username uses disallowed characters. Allowed: Alphabetic, Numeric, and Underscores";
        return HttpResponse::BadRequest().json(json!({"reason": reason}))
    }
    if details.password.invalid_password() {
        let reason = "Password uses disallowed characters. Allowed: Alphabetic, Numeric, Graphic (0x21-0x7E)";
        return HttpResponse::BadRequest().json(json!({"reason": reason}))
    }
    // send to DB
    let result = db.create_account(&details.username, &details.password).await;
    match result {
        Ok(_)  => HttpResponse::Created().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::BadRequest().json(json!({"reason": "Username already taken"}))
    }
}

#[post("/authenticate")]
pub async fn authenticate(db: web::Data<Database>, login_details: web::Json<Login>) -> HttpResponse {
    // Username and password validity checks
    if login_details.username.invalid_username() {
        let reason = "Username uses disallowed characters. Allowed: Alphabetic, Numeric, and Underscores";
        return HttpResponse::BadRequest().json(json!({"reason": reason}))
    }
    if login_details.password.invalid_password() {
        let reason = "Password uses disallowed characters. Allowed: Alphabetic, Numeric, Graphic (0x21-0x7E)";
        return HttpResponse::BadRequest().json(json!({"reason": reason}))
    }
    // Login in DB and get token
    let auth = db.login(&login_details.username, &login_details.password).await;
    match auth {
        Ok(token) => HttpResponse::Ok().json(LoginToken { token }),
        Err(_) => HttpResponse::Unauthorized().json(json!({"reason": "Invalid login details"}))
    }
}

#[post("/message/{chat_id}")]
pub async fn send_message(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<NewMessage>,
    auth: BearerAuth
) -> HttpResponse {
    let token = auth.token();
    let uid = match db.token_to_uid(&token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"reason": "Invalid or expired token"}))
    };
    let chat_id = path.to_owned();
    let status = db.add_message(&chat_id, &uid, &body.content).await;
    match status {
        Ok(_)  => HttpResponse::Ok().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"reason": "Sending failure"}))
    }
}

#[get("/message")]
pub async fn get_messages(
    db: web::Data<Database>,
    body: web::Json<MessageRequest>,
    auth: BearerAuth
) -> HttpResponse {
    let from = body.from_time;
    match db.token_to_uid(auth.token()).await {
        Ok(uid) => HttpResponse::Ok().json(db.get_messages(&uid, from).await.unwrap()),
        Err(_) => HttpResponse::Unauthorized().json(json!({"reason": "User not logged in or bad token"}))
    }
}

#[get("/message/{chat_id}")]
pub async fn get_conversation_conversation(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<MessageRequest>,
    auth: BearerAuth
) -> HttpResponse {
    let cid = path.to_owned();
    let from = body.from_time;
    match db.token_to_uid(auth.token()).await {
        Ok(uid) => HttpResponse::Ok().json(db.get_conversation_messages(&uid, &cid, from).await),
        Err(_) => HttpResponse::Unauthorized().json(json!({"reason": "User not logged in or bad token"}))
    }
}

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(health)
            .service(register)
            .service(authenticate)
            .service(send_message)
            .service(get_messages)
            .service(get_conversation_conversation)
    );
}