use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::project::MemberRole;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProjectMemberDetails {
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub role: MemberRole,
    pub invited_at: DateTime<Utc>,
    pub joined_at: Option<DateTime<Utc>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberInput {
    pub email: String,
    pub role: MemberRole,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleInput {
    pub role: MemberRole,
}
