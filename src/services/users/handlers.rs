use actix_web::{HttpResponse, Responder};

pub async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("List of items")
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Item created")
}
