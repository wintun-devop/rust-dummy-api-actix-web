use actix_web::{App, HttpServer};

mod handlers;
mod utils;

use handlers::health_check::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(health_check) // register the handler
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
