use actix_web::{App, HttpServer};

mod handlers;
mod utils;
mod config;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(handlers::handler_config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}