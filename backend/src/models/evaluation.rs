use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RiskEvaluation {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub severity: i32,
    pub likelihood: i32,
    pub detectability: i32,
    pub score: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
