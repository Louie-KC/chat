use actix_web::{get, web::ServiceConfig, HttpResponse};
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


use crate::{database::DatabaseServiceError, DatabaseService};


pub fn config(config: &mut ServiceConfig) -> () {
    config.service(actix_web::web::scope("")
        .service(health)
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

