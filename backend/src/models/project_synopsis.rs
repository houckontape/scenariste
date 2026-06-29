use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProjectSynopsis {
    pub id: Uuid,
    pub project_id: Uuid,
    pub logline: Option<String>,
    pub summary_short: Option<String>,
    pub summary_long: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectSynopsisInput {
    pub logline: Option<String>,
    pub summary_short: Option<String>,
    pub summary_long: Option<String>,
}
