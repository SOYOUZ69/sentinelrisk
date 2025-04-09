use actix_web::{get, post, web, HttpResponse, Responder, put, delete};
use sqlx::PgPool;
use crate::models::incident::Incident;

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewIncident {
    pub title: String,
    pub description: Option<String>,
    pub severity: String,
    pub status: String,
    pub related_risk_id: Option<Uuid>,
}

#[get("/incidents")]
pub async fn get_all_incidents(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Incident,
        r#"
        SELECT id, title, description, severity, status, related_risk_id, created_at
        FROM incidents
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(incidents) => HttpResponse::Ok().json(incidents),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des incidents: {}", e);
            HttpResponse::InternalServerError().body("Erreur serveur")
        }
    }
}

#[get("/incidents/{id}")]
pub async fn get_incident_by_id(
    db_pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Incident,
        r#"
        SELECT id, title, description, severity, status, related_risk_id, created_at
        FROM incidents
        WHERE id = $1
        "#,
        *id
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(incident)) => HttpResponse::Ok().json(incident),
        Ok(None) => HttpResponse::NotFound().body("Incident non trouvé"),
        Err(e) => {
            eprintln!("Erreur : {}", e);
            HttpResponse::InternalServerError().body("Erreur serveur")
        }
    }
}

#[post("/incidents")]
pub async fn create_incident(
    db_pool: web::Data<PgPool>,
    incident_data: web::Json<NewIncident>,
) -> impl Responder {
    let new_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO incidents (id, title, description, severity, status, related_risk_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        new_id,
        incident_data.title,
        incident_data.description,
        incident_data.severity,
        incident_data.status,
        incident_data.related_risk_id,
        now
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "id": new_id })),
        Err(e) => {
            eprintln!("Erreur lors de la création de l'incident: {}", e);
            HttpResponse::InternalServerError().body("Erreur lors de la création")
        }
    }
}

#[put("/incidents/{id}")]
pub async fn update_incident(
    db_pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    updated_data: web::Json<NewIncident>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE incidents
        SET title = $1, description = $2, severity = $3, status = $4, related_risk_id = $5
        WHERE id = $6
        "#,
        updated_data.title,
        updated_data.description,
        updated_data.severity,
        updated_data.status,
        updated_data.related_risk_id,
        *id
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() == 1 => HttpResponse::Ok().body("Incident mis à jour"),
        Ok(_) => HttpResponse::NotFound().body("Incident introuvable"),
        Err(e) => {
            eprintln!("Erreur lors de la mise à jour : {}", e);
            HttpResponse::InternalServerError().body("Erreur serveur")
        }
    }
}

#[delete("/incidents/{id}")]
pub async fn delete_incident(
    db_pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM incidents
        WHERE id = $1
        "#,
        *id
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() == 1 => HttpResponse::Ok().body("Incident supprimé"),
        Ok(_) => HttpResponse::NotFound().body("Incident introuvable"),
        Err(e) => {
            eprintln!("Erreur lors de la suppression : {}", e);
            HttpResponse::InternalServerError().body("Erreur serveur")
        }
    }
}
