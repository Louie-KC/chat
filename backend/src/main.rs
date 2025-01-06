mod handler;

use actix_web::{App, HttpServer};
use log::info;

const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8000;

pub struct AppState {
    // TODO: Common actor items (e.g. DB connection)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    let app = HttpServer::new(move ||
        App::new()
            .configure(handler::config)
            .app_data(actix_web::web::Data::new(AppState {}))
    ).bind((SERVER_ADDR, SERVER_PORT))?;

    env_logger::init();
    info!("Server running at http://{}:{}", SERVER_ADDR, SERVER_PORT);
    
    app.run().await
}
