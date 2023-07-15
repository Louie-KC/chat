mod api;
mod models;
mod database;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use crate::database::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::new(db_url.as_str()).await;
    let data = web::Data::new(db);

    let server_address = "0.0.0.0";  // 0.0.0.0 Allows for devices on LAN to find server
    let server_port = 8080;
    let app = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(api::api::config)
    })
    .bind((server_address, server_port))?
    .run();
    println!("Server running at http://{server_address}:{server_port}/");
    app.await
}
