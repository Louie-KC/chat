use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

use crate::{
    models::{
        message::Message,
        authenticate::{Login, LoginToken},
        account::Account
    },
    database::database::Database
};

#[post("/register")]
pub async fn register(db: web::Data<Database>, details: web::Json<Account>) -> HttpResponse {
    let result = db.create_account(&details.username, &details.password);
    match result {
        Ok(_)  => HttpResponse::Created().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::BadRequest().body(format!("Username {} already taken", details.username))
    }
}

#[post("/authenticate")]
pub async fn authenticate(db: web::Data<Database>, login_details: web::Json<Login>) -> HttpResponse {
    let auth = db.login(&login_details.username, &login_details.password);
    match auth {
        Ok(token) => HttpResponse::Ok().json(LoginToken { token }),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[post("/message/{user_id}")]
pub async fn send_message(
    db: web::Data<Database>,
    body: web::Json<Message>,
    path: web::Path<String>,
    auth: BearerAuth
) -> HttpResponse {
    let uid= path.into_inner();
    let token = auth.token();
    if !db.valid_token(&token, &uid) {
        return HttpResponse::Unauthorized().body("Invalid user id and token combination")
    }
    let status = db.add_message(&body.to, &body.from, &body.content);
    match status {
        Ok(_)  => HttpResponse::Ok().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::InternalServerError().body("Sending failure")
    }
}

#[get("/message/{user_id}")]
pub async fn get_messages(
    db: web::Data<Database>,
    path: web::Path<String>,
    auth: BearerAuth
) -> HttpResponse {
    let user_id = path.into_inner();
    match db.valid_token(auth.token(), &user_id) {
        true  => HttpResponse::Ok().json(db.get_messages_brief(&user_id)),
        false => HttpResponse::Unauthorized().body("User not logged in or bad token")
    }
}

#[get("/message/{user_id}/{recipient_id}")]
pub async fn get_conversation(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    auth: BearerAuth
) -> HttpResponse {
    let (user_id, recipient_id) = path.into_inner();
    match db.valid_token(auth.token(), &user_id) {
        true  => HttpResponse::Ok().json(db.get_conversation_messages(&user_id, &recipient_id)),
        false => HttpResponse::Unauthorized().body("User not logged in or bad token")
    }
}

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(register)
            .service(authenticate)
            .service(send_message)
            .service(get_messages)
            .service(get_conversation)
    );
}