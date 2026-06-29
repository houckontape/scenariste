// backend/src/models/user.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    FreeUser,
    PremiumUser,
    Support,
    Admin,
    SuperAdmin,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)] // Sécurité SaaS : Ne jamais exposer le hash
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Structure de réception pour l'inscription depuis Angular
#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: std::string::String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserWithProfile {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    pub profile: Option<Profile>,
}