use actix_web::{get, post, web, HttpResponse};

use crate::{models::{message::Message, authenticate::{Login, LoginToken}}, database::database::Database};

#[post("/authenticate")]
pub async fn authenticate(db: web::Data<Database>, login_details: web::Json<Login>) -> HttpResponse {
    let auth = db.login(login_details.username.clone(), login_details.password.clone());
    match auth {
        Ok(token) => HttpResponse::Ok().json(LoginToken { token }),
        Err(_) => HttpResponse::BadRequest().body("Invalid login")
    }
}

#[post("/message")]
pub async fn send_message(db: web::Data<Database>, message: web::Json<Message>) -> HttpResponse {
    let status = db.add_message(message.to.clone(), message.from.clone(), message.content.clone());
    match status {
        Ok(_)  => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Sending failure")
    }
}

#[get("/message/{user_id}")]
pub async fn get_messages(db: web::Data<Database>, path: web::Path<String>) -> HttpResponse {
// pub async fn get_messages(db: web::Data<Database>, path: web::Path<String>, token: web::Data<LoginToken>) -> HttpResponse {
    // let messages = db.get_messages_brief(bearer.token.clone());
    let user_id = path.into_inner();

    let messages = db.get_messages_brief(&user_id);
    HttpResponse::Ok().json(messages)

    // match db.valid_token(&token.token, &user_id) {
    //     true  => HttpResponse::Ok().json(db.get_messages_brief(user_id)),
    //     false => HttpResponse::BadRequest().body("User not logged in or bad token")
    // }
    
}

#[get("/message/{user_id}/{recipient_id}")]
pub async fn get_conversation(db: web::Data<Database>, path: web::Path<(String, String)>) -> HttpResponse {
// pub async fn get_conversation(db: web::Data<Database>, path: web::Path<(String, String)>, token: web::Data<LoginToken>) -> HttpResponse {
    let (user_id, recipient_id) = path.into_inner();

    let messages = db.get_conversation_messages(&user_id, &recipient_id);
    HttpResponse::Ok().json(messages)

    // match db.valid_token(&token.token, &user_id) {
    //     true  => HttpResponse::Ok().json(db.get_conversation_messages(user_id, recipient_id)),
    //     false => HttpResponse::BadRequest().body("User not logged in or bad token")
    // }
}

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(authenticate)
            .service(send_message)
            .service(get_messages)
            .service(get_conversation)
    );
}