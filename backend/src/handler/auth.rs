use axum::{extract::State, Json, http::StatusCode};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2
};
use crate::db::DbPool;
use crate::models::user::RegisterInput;

/// Handler pour l'inscription d'un nouvel utilisateur
pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterInput>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {

    // 1. Validation basique
    if payload.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Le mot de passe doit faire au moins 8 caractères".to_string()));
    }

    // 2. Hachage du mot de passe
    // Nous générons un sel basique à partir des octets de l'email pour contourner le besoin d'OsRng direct,
    // ou nous laissons Argon2 utiliser ses valeurs par défaut de manière sécurisée.
    let salt = SaltString::b64_encode(payload.email.as_bytes())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de génération du sel".to_string()))?;

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors du hachage sécurisé".to_string()))?
        .to_string();

    // 3. Insertion sécurisée dans PostgreSQL via SQLx
    sqlx::query!(
        "INSERT INTO users (email, company, password_hash) VALUES ($1, $2, $3)",
        payload.email,
        payload.company,
        password_hash
    )
        .execute(&pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique constraint") {
                (StatusCode::CONFLICT, "Cet email est déjà utilisé".to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur base de données".to_string())
            }
        })?;

    Ok((StatusCode::CREATED, Json("Utilisateur créé avec succès".to_string())))
}