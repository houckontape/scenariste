use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BrainstormingNote {
    pub id: Uuid,
    pub project_id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBrainstormingNoteInput {
    pub project_id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBrainstormingNoteInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}
