use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

// Dummy store
fn dummy_items() -> Vec<Item> {
    vec![
        Item { id: 1, name: "Item One".to_string() },
        Item { id: 2, name: "Item Two".to_string() },
    ]
}

#[get("/items")]
async fn list_items() -> impl Responder {
    HttpResponse::Ok().json(dummy_items())
}

#[get("/items/{id}")]
async fn get_item(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let item = dummy_items().into_iter().find(|i| i.id == id);
    match item {
        Some(i) => HttpResponse::Ok().json(i),
        None => HttpResponse::NotFound().json(json!({"error": "Item not found"})),
    }
}

#[post("/items")]
async fn create_item(new_item: web::Json<Item>) -> impl Responder {
    HttpResponse::Created().json(json!({
        "status": "created",
        "item": new_item.into_inner()
    }))
}

#[put("/items/{id}")]
async fn update_item(path: web::Path<u32>, updated: web::Json<Item>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "status": "updated",
        "id": id,
        "item": updated.into_inner()
    }))
}

#[delete("/items/{id}")]
async fn delete_item(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "status": "deleted",
        "id": id
    }))
}

// Expose configure function
pub fn items_route_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_items)
       .service(get_item)
       .service(create_item)
       .service(update_item)
       .service(delete_item);
}
