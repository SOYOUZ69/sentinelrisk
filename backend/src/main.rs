mod db;
mod controllers;
mod models;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenvy::dotenv;
use controllers::health_controller::health_check;

use controllers::risk_controller::get_all_risks;
use controllers::risk_controller::{create_risk, update_risk, delete_risk, update_risk_status, get_risk_history, create_evaluation, get_evaluation, get_critical_risks};
use actix_web::web;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("SentinelRisk Backend API 🚀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Starting server at http://localhost:{port}");

    // Connexion DB
    let pool = match db::connect_db().await {
        Ok(p) => {
            println!("✅ Connexion à PostgreSQL réussie");
            p
        }
        Err(e) => {
            println!("❌ Erreur de connexion : {:?}", e);
            return Ok(());
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .service(get_all_risks)
            .service(create_risk)
            .service(update_risk)
            .service(delete_risk)
            .service(update_risk_status)
            .service(get_risk_history)
            .service(create_evaluation)
            .service(get_evaluation)
            .service(get_critical_risks)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}