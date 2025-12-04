use actix_web::web;

pub mod health_check;
pub mod items;


pub fn handler_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health_check::health_check)
            .configure(items::items_route_configure)
    );
}