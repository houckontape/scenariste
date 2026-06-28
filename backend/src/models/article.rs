use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String, // Contiendra le Markdown brut
    pub author_name: String,
    pub published_at: DateTime<Utc>,
}