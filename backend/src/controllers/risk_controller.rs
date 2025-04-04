#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CriticalRisk {
    pub id: Uuid,
    pub title: String,
    pub status: RiskStatus,
    pub score: i32,
}

use actix_web::{get, web, HttpResponse, Responder, post, put, delete, patch};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::risk::{Risk, NewRisk, DbRisk, RiskStatus};
use crate::models::risk_history::RiskStatusHistory;
use crate::models::evaluation::RiskEvaluation;
use actix_web::web::Json;
use serde::Deserialize;
use serde::Serialize;
use chrono::Utc;

#[derive(Deserialize)]
pub struct UpdateStatusPayload {
    pub status: RiskStatus,
}

fn is_valid_transition(current: &RiskStatus, next: &RiskStatus) -> bool {
    matches!(
        (current, next),
        (RiskStatus::Identified, RiskStatus::Assessed)
        | (RiskStatus::Assessed, RiskStatus::InTreatment)
        | (RiskStatus::InTreatment, RiskStatus::Monitoring)
        | (RiskStatus::Monitoring, RiskStatus::Accepted)
        | (RiskStatus::Monitoring, RiskStatus::Rejected)
        | (RiskStatus::Monitoring, RiskStatus::Transferred)
        | (RiskStatus::InTreatment, RiskStatus::Closed)
    )
}

#[get("/risks")]
pub async fn get_all_risks(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, DbRisk>("SELECT * FROM risks")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(db_risks) => {
            let risks: Vec<Risk> = db_risks
                .into_iter()
                .map(|r| Risk {
                    score: Some(r.impact * r.probability),
                    id: r.id,
                    title: r.title,
                    description: r.description,
                    impact: r.impact,
                    probability: r.probability,
                    status: r.status,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                })
                .collect();

            HttpResponse::Ok().json(risks)
        },
        Err(err) => {
            eprintln!("Erreur lors de la récupération des risques : {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/risks")]
pub async fn create_risk(
    db_pool: web::Data<PgPool>,
    risk: web::Json<NewRisk>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DbRisk>(
        r#"
        INSERT INTO risks (title, description, impact, probability)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#
    )
    .bind(&risk.title)
    .bind(&risk.description)
    .bind(risk.impact)
    .bind(risk.probability)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(db_risk) => {
            let full_risk = Risk {
                id: db_risk.id,
                title: db_risk.title,
                description: db_risk.description,
                impact: db_risk.impact,
                probability: db_risk.probability,
                status: db_risk.status,
                created_at: db_risk.created_at,
                updated_at: db_risk.updated_at,
                score: Some(db_risk.impact * db_risk.probability),
            };
            HttpResponse::Ok().json(full_risk)
        }
        Err(e) => {
            eprintln!("Erreur création risque : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/risks/{id}")]
pub async fn update_risk(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    updated_risk: web::Json<NewRisk>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query_as::<_, Risk>(
        r#"
        UPDATE risks
        SET title = $1,
            description = $2,
            impact = $3,
            probability = $4,
            updated_at = NOW()
        WHERE id = $5
        RETURNING *;
        "#,
    )
    .bind(&updated_risk.title)
    .bind(&updated_risk.description)
    .bind(updated_risk.impact)
    .bind(updated_risk.probability)
    .bind(id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(risk) => HttpResponse::Ok().json(risk),
        Err(e) => {
            eprintln!("Erreur mise à jour : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/risks/{id}")]
pub async fn delete_risk(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM risks WHERE id = $1")
        .bind(id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Risque supprimé"),
        Err(e) => {
            eprintln!("Erreur suppression : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[patch("/risks/{id}/status")]
pub async fn update_risk_status(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateStatusPayload>,
) -> impl Responder {
    let id = path.into_inner();

    let current_status = sqlx::query_scalar::<_, RiskStatus>(
        "SELECT status FROM risks WHERE id = $1"
    )
    .bind(id)
    .fetch_one(db_pool.get_ref())
    .await;

    let current_status = match current_status {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Erreur récupération statut courant : {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if !is_valid_transition(&current_status, &payload.status) {
        return HttpResponse::BadRequest().body("Transition de statut non autorisée");
    }

    let result = sqlx::query_as::<_, DbRisk>(
        r#"
        UPDATE risks
        SET status = $1,
            updated_at = now()
        WHERE id = $2
        RETURNING *;
        "#,
    )
    .bind(&payload.status)
    .bind(id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(db_risk) => {
            let risk = Risk {
                score: Some(db_risk.impact * db_risk.probability),
                id: db_risk.id,
                title: db_risk.title,
                description: db_risk.description,
                impact: db_risk.impact,
                probability: db_risk.probability,
                status: db_risk.status,
                created_at: db_risk.created_at,
                updated_at: db_risk.updated_at,
            };

            // Historiser le changement de statut
            let _ = sqlx::query(
                r#"
                INSERT INTO risk_status_history (risk_id, old_status, new_status)
                VALUES ($1, $2, $3);
                "#
            )
            .bind(id)
            .bind(current_status.to_string())
            .bind(payload.status.to_string())
            .execute(db_pool.get_ref())
            .await;

            HttpResponse::Ok().json(risk)
        }
        Err(e) => {
            eprintln!("Erreur mise à jour statut : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/risks/{id}/history")]
pub async fn get_risk_history(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query_as::<_, RiskStatusHistory>(
        r#"
        SELECT * FROM risk_status_history
        WHERE risk_id = $1
        ORDER BY changed_at ASC;
        "#
    )
    .bind(id)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => {
            eprintln!("Erreur récupération historique : {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
pub struct RiskEvaluationInput {
    pub severity: i32,
    pub likelihood: i32,
    pub detectability: i32,
}

#[post("/risks/{id}/evaluation")]
pub async fn create_evaluation(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<RiskEvaluationInput>,
) -> impl Responder {
    let risk_id = path.into_inner();

    let result = sqlx::query_as::<_, RiskEvaluation>(
        r#"
        INSERT INTO risk_evaluation (risk_id, severity, likelihood, detectability)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#
    )
    .bind(risk_id)
    .bind(payload.severity)
    .bind(payload.likelihood)
    .bind(payload.detectability)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(evaluation) => HttpResponse::Ok().json(evaluation),
        Err(err) => {
            eprintln!("Erreur insertion évaluation : {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/risks/{id}/evaluation")]
pub async fn get_evaluation(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let risk_id = path.into_inner();

    let result = sqlx::query_as::<_, RiskEvaluation>(
        r#"
        SELECT * FROM risk_evaluation
        WHERE risk_id = $1;
        "#
    )
    .bind(risk_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(evaluation)) => HttpResponse::Ok().json(evaluation),
        Ok(None) => HttpResponse::NotFound().body("Aucune évaluation trouvée"),
        Err(err) => {
            eprintln!("Erreur récupération évaluation : {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/risks/critical")]
pub async fn get_critical_risks(
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query_as::<_, CriticalRisk>(
        r#"
        SELECT r.id, r.title, r.status, e.score
        FROM risks r
        JOIN risk_evaluation e ON r.id = e.risk_id
        ORDER BY e.score DESC;
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(risks) => HttpResponse::Ok().json(risks),
        Err(err) => {
            eprintln!("Erreur récupération risques critiques : {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}