use actix_web::web;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Forbidden().body("Forbidden")
}

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(index);
}
