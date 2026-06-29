use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "project_type", rename_all = "snake_case")]
pub enum ProjectType {
    Movie,
    Series,
    Animation,
    Novel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "member_role", rename_all = "snake_case")]
pub enum MemberRole {
    Owner,
    Write,
    Read,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub p_type: ProjectType,
    pub is_private: bool,
    pub shooting_technique_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectInput {
    pub title: String,
    pub description: Option<String>,
    pub p_type: ProjectType,
    pub is_private: bool,
    pub shooting_technique_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub p_type: Option<ProjectType>,
    pub is_private: Option<bool>,
    pub shooting_technique_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProjectWithRole {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub p_type: ProjectType,
    pub is_private: bool,
    pub shooting_technique_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_role: MemberRole,
}
