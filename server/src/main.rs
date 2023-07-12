mod api;
mod models;
mod database;

use actix_web::{web, App, HttpServer};

use crate::database::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();
    let data = web::Data::new(db);

    let server_address = "127.0.0.1";
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
