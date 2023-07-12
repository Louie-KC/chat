use actix_web::{get, post, web, HttpResponse};

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
    let result = db.create_account(details.username.clone(), details.password.clone());
    match result {
        Ok(_)  => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::BadRequest().body(format!("Username {} already taken", &details.username))
    }
}

#[post("/authenticate")]
pub async fn authenticate(db: web::Data<Database>, login_details: web::Json<Login>) -> HttpResponse {
    let auth = db.login(login_details.username.clone(), login_details.password.clone());
    match auth {
        Ok(token) => HttpResponse::Ok().json(LoginToken { token }),
        Err(_) => HttpResponse::BadRequest().body("Invalid login")
    }
}

#[post("/message/{user_id}/{token}")]
pub async fn send_message(db: web::Data<Database>, body: web::Json<Message>, path: web::Path<(String, String)>) -> HttpResponse {
    let (uid, token) = path.into_inner();
    if !db.valid_token(&token, &uid) {
        return HttpResponse::BadRequest().body("Invalid user id and token combination")
    }
    let status = db.add_message(body.to.clone(), body.from.clone(), body.content.clone());
    match status {
        Ok(_)  => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Sending failure")
    }
}

#[get("/message/{user_id}/{token}")]
pub async fn get_messages(db: web::Data<Database>, path: web::Path<(String, String)>) -> HttpResponse {
    let (user_id, token) = path.into_inner();
    match db.valid_token(&token, &user_id) {
        true  => HttpResponse::Ok().json(db.get_messages_brief(&user_id)),
        false => HttpResponse::BadRequest().body("User not logged in or bad token")
    }
}

#[get("/message/{user_id}/{token}/{recipient_id}")]
pub async fn get_conversation(db: web::Data<Database>, path: web::Path<(String, String, String)>) -> HttpResponse {
    let (user_id, token, recipient_id) = path.into_inner();
    match db.valid_token(&token, &user_id) {
        true  => HttpResponse::Ok().json(db.get_conversation_messages(&user_id, &recipient_id)),
        false => HttpResponse::BadRequest().body("User not logged in or bad token")
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