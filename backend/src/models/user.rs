use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub company: String,
    // On ne sérialise jamais le hash du mot de passe vers le frontend pour des raisons de sécurité
    #[serde(skip_serializing)]
    pub password_hash: String,
}

/// Structure reçue lors de l'inscription (Register)
#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub company: String,
    pub password: String,
}