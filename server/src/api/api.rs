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

#[get("/message")]
pub async fn get_messages(db: web::Data<Database>, bearer: web::Json<LoginToken>) -> HttpResponse {
    let messages = db.get_messages(bearer.token.clone());
    match messages {
        Ok(msgs) => HttpResponse::Ok().json(msgs),
        Err(_) => HttpResponse::BadRequest().body("User not logged in")
    }
}

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(authenticate)
            .service(send_message)
            .service(get_messages)
    );
}