use actix_web::web;

pub mod health_check;
pub mod items;
use crate::config;


pub fn handler_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&config::config().api_base_path)
            .service(health_check::health_check)
            .configure(items::items_route_configure)
    );
}