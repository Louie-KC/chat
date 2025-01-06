use actix_web::{get, web::ServiceConfig, HttpResponse};

pub fn config(config: &mut ServiceConfig) -> () {
    config.service(actix_web::web::scope("/")
        .service(health)
    );
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

