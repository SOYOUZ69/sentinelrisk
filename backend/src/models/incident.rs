use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Incident {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub severity: String,         // Exemple : "Faible", "Moyenne", "Critique"
    pub status: String,           // Exemple : "Nouveau", "En cours", "Résolu"
    pub related_risk_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}
