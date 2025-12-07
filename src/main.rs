use actix_web::{App, HttpServer};

mod config;
mod handlers;
mod utils;
use config::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let build_url = config().build_address;
    let port_number = config().build_port;
    let port: u16 = port_number.parse().expect("invalid port");
    println!("🚀 Running at {}:{}.", build_url,port);
    HttpServer::new(|| App::new().configure(handlers::handler_config))
        .bind((build_url,port))?
        .run()
        .await
}
