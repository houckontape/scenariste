// backend/src/models/article.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Article {
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
pub struct CreateArticleInput {
    pub project_id: Uuid, // Requis par la table brainstorming_notes
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
