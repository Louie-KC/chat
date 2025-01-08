mod database;
mod handler;
mod models;

use dotenv::dotenv;

use actix_web::{
    App,
    HttpServer
};
use argon2::Argon2;
use database::DatabaseService;
use log::info;

const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8000;

fn get_var_or_panic(var_name: &str) -> String {
    std::env::var(var_name).expect(&format!("{} is not set", var_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    dotenv().ok();

    let db_service = DatabaseService::new(&get_var_or_panic("DATABASE_URL")).await;
    let argon2 = Argon2::default();
    
    let db_service_data = actix_web::web::Data::new(db_service);
    let argon2_data = actix_web::web::Data::new(argon2);

    let app = HttpServer::new(move ||
        App::new()
            .configure(handler::config)
            .app_data(db_service_data.clone())
            .app_data(argon2_data.clone())
    ).bind((SERVER_ADDR, SERVER_PORT))?;

    env_logger::init();
    info!("Server running at http://{}:{}", SERVER_ADDR, SERVER_PORT);
    
    app.run().await
}
