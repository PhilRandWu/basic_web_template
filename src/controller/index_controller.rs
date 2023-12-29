use actix_web::{web, get, Responder, HttpResponse};
use crate::AppState;
use log::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
}

#[get("/status")]
async fn status(data: web::Data<AppState<'_>>) -> impl Responder {
    info!("GET: /status connections: {:?}",&data.connections);
    HttpResponse::Ok().body("I an up")
}