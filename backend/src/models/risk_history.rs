use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RiskStatusHistory {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub old_status: String,
    pub new_status: String,
    pub changed_at: DateTime<Utc>,
}
