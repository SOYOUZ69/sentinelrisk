mod db;
mod controllers;
mod models;

use actix_web::{get, put, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenvy::dotenv;
use controllers::health_controller::health_check;

use controllers::risk_controller::get_all_risks;
use controllers::risk_controller::{create_risk, update_risk, delete_risk, update_risk_status, get_risk_history, create_evaluation, get_evaluation, get_critical_risks, get_risk_by_id};
use controllers::incident_handler::{get_all_incidents, create_incident, get_incident_by_id, update_incident, delete_incident}; // Importation de delete_incident
use actix_web::web;
use actix_cors::Cors;
use actix_web::http::header;

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
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .max_age(3600),
            )
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
            .service(create_incident) // Ajout de create_incident
            .service(get_all_incidents) // Ajout de get_all_incidents
            .service(get_incident_by_id) // Ajout de get_incident_by_id
            .service(update_incident) // Ajout de update_incident
            .service(delete_incident) // Ajout de delete_incident
            .service(get_risk_by_id)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}