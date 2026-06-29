use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CharacterBible {
    pub id: Uuid,
    pub project_id: Uuid,
    pub full_name: String,
    pub character_role: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub attributes: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCharacterInput {
    pub project_id: Uuid,
    pub full_name: String,
    pub character_role: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub attributes: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCharacterInput {
    pub full_name: Option<String>,
    pub character_role: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub attributes: Option<Value>,
}
