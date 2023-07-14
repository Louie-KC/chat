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

#[post("/register")]
pub async fn register(db: web::Data<Database>, details: web::Json<Account>) -> HttpResponse {
    let result = db.create_account(&details.username, &details.password).await;
    match result {
        Ok(_)  => HttpResponse::Created().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::BadRequest().body(format!("Username {} already taken", details.username))
    }
}

#[post("/authenticate")]
pub async fn authenticate(db: web::Data<Database>, login_details: web::Json<Login>) -> HttpResponse {
    let auth = db.login(&login_details.username, &login_details.password).await;
    match auth {
        Ok(token) => HttpResponse::Ok().json(LoginToken { token }),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

// #[post("/message/{user_id}")]
// pub async fn send_message(
//     db: web::Data<Database>,
//     body: web::Json<Message>,
//     path: web::Path<String>,
//     auth: BearerAuth
// ) -> HttpResponse {
//     let uid= path.into_inner();
//     let token = auth.token();
//     if !db.valid_token(&token, &uid).await {
//         return HttpResponse::Unauthorized().body("Invalid user id and token combination")
//     }
//     let status = db.add_message(&body.to, &body.from, &body.content).await;
//     match status {
//         Ok(_)  => HttpResponse::Ok().json(json!({"status": "Success"})),
//         Err(_) => HttpResponse::InternalServerError().body("Sending failure")
//     }
// }

#[post("/message")]
pub async fn send_message(
    db: web::Data<Database>,
    body: web::Json<Message>,
    auth: BearerAuth
) -> HttpResponse {
    // let uid= path.into_inner();
    let token = auth.token();
    let uid = match db.token_to_uid(&token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token")
    };
    // println!("calling add_message: {}, {}, {}", &body.chat_id, &uid, &body.content);
    let status = db.add_message(&body.chat_id, &uid, &body.content).await;
    match status {
        Ok(_)  => HttpResponse::Ok().json(json!({"status": "Success"})),
        Err(_) => HttpResponse::InternalServerError().body("Sending failure")
    }
}

#[get("/message")]
pub async fn get_messages(
    db: web::Data<Database>,
    auth: BearerAuth
) -> HttpResponse {
    match db.token_to_uid(auth.token()).await {
        Ok(uid) => HttpResponse::Ok().json(db.get_messages(&uid).await),
        Err(_) => return HttpResponse::Unauthorized().body("User not logged in or bad token")
    }
}

#[get("/message")]
pub async fn get_conversation(
    db: web::Data<Database>,
    body: web::Json<MessageRequest>,
    auth: BearerAuth
) -> HttpResponse {
    match db.token_to_uid(auth.token()).await {
        Ok(uid) => HttpResponse::Ok().json(db.get_conversation_messages(&uid, &body.chat_id).await),
        Err(_) => HttpResponse::Unauthorized().body("User not logged in or bad token")
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