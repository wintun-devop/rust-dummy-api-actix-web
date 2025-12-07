use actix_web::web;

pub mod health_check;
pub mod items;
use crate::config::config;

pub fn handler_config(cfg: &mut web::ServiceConfig) {
    let api_base_path = config().api_base_path;
    cfg.service(
        web::scope(&api_base_path)
            .service(health_check::health_check)
            .configure(items::items_route_configure),
    );
}
