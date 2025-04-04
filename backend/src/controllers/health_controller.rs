use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("✅ SentinelRisk API is running!")
}