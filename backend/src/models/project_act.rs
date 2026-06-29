use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProjectAct {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub position: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectActInput {
    pub project_id: Uuid,
    pub title: String,
    pub position: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectActInput {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub description: Option<String>,
}
