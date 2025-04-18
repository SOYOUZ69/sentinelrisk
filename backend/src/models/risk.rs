use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "PascalCase")]
pub enum RiskStatus {
    Identified,
    Assessed,
    InTreatment,
    Monitoring,
    Accepted,
    Rejected,
    Transferred,
    Closed,
}

impl fmt::Display for RiskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            RiskStatus::Identified => "Identified",
            RiskStatus::Assessed => "Assessed",
            RiskStatus::InTreatment => "InTreatment",
            RiskStatus::Monitoring => "Monitoring",
            RiskStatus::Accepted => "Accepted",
            RiskStatus::Rejected => "Rejected",
            RiskStatus::Transferred => "Transferred",
            RiskStatus::Closed => "Closed",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, FromRow)]
pub struct DbRisk {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub impact: i32,
    pub probability: i32,
    pub status: RiskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub external_id: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub regulation: Option<String>,
    pub control_measure_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Risk {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub impact: i32,
    pub probability: i32,
    pub status: RiskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub external_id: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub regulation: Option<String>,
    pub control_measure_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>, // <-- Calculé manuellement

}

#[derive(Debug, Deserialize)]
pub struct NewRisk {
    pub title: String,
    pub description: Option<String>,
    pub external_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub risk_map: Option<String>,
    pub threat_map: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub regulation: Option<String>,
    pub control_measure_id: Option<String>,
    pub assets: Option<Vec<String>>,
    pub technology: Option<String>,
    pub team: Option<String>,
    pub stakeholders: Option<Vec<String>>,
    pub owner: Option<String>,
    pub owner_delegate: Option<String>,
    pub source: Option<String>,
    pub evaluation_method: Option<String>,
    pub impact: i32,
    pub probability: i32,
    pub evaluation_notes: Option<String>,
    pub additional_notes: Option<String>,
    pub file_attachment: Option<String>,
    pub status: Option<RiskStatus>,
}