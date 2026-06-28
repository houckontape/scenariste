use axum::{
    extract::{State, FromRequestParts},
    http::{StatusCode, request::Parts, header::{AUTHORIZATION}},
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::auth::{Claims, AuthResponse, LoginInput};
use crate::models::user::{RegisterInput, UserWithProfile, Profile, UpdateProfileInput, UserRole};
use crate::service::auth_service::{AuthService, AuthServiceError};

// Extracteur JWT : FromRequestParts permet d'injecter Claims dans n'importe quel handler protégé
#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extraire le header Authorization
        let auth_header = parts.headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Token manquant".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Format de token invalide".to_string()));
        }

        let token = &auth_header[7..];

        // 2. Décoder et valider le JWT
        let secret = std::env::var("JWT_SECRET").map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "JWT_SECRET non configuré".to_string()))?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token invalide ou expiré".to_string()))?;

        Ok(token_data.claims)
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    match AuthService::login(&pool, &payload.email, &payload.password).await {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(err) => match err {
            AuthServiceError::InvalidCredentials =>
                Err((StatusCode::UNAUTHORIZED, "Email ou mot de passe incorrect".to_string())),
            AuthServiceError::DatabaseError(e) =>
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e))),
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne".to_string())),
        }
    }
}

pub async fn protected_route(claims: Claims) -> impl IntoResponse {
    (StatusCode::OK, format!("Succès ! Bienvenue utilisateur {}, votre rôle est {:?}", claims.sub, claims.role))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    match AuthService::register_user(&pool, &payload.email, &payload.password).await {
        Ok(user_id) => {
            Ok((StatusCode::CREATED, Json(serde_json::json!({
                "message": "Utilisateur et profil créés avec succès !",
                "user_id": user_id
            }))))
        },
        Err(err) => match err {
            AuthServiceError::PasswordTooShort =>
                Err((StatusCode::BAD_REQUEST, "Le mot de passe doit faire au moins 8 caractères.".to_string())),
            AuthServiceError::EmailConflict =>
                Err((StatusCode::CONFLICT, "Cet email est déjà utilisé.".to_string())),
            AuthServiceError::EntropyError =>
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Erreur de sécurité interne (Chiffrement)".to_string())),
            AuthServiceError::DatabaseError(e) =>
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur interne : {}", e))),
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne".to_string())),
        }
    }
}

pub async fn get_current_user(
    State(pool): State<PgPool>,
) -> Result<Json<UserWithProfile>, (StatusCode, String)> {
    // Pour l'instant, on simule un ID utilisateur en attendant JWT.
    // On va chercher le premier utilisateur de la base pour la démo.
    let user_row = sqlx::query!(
        r#"
        SELECT
            u.id as user_id, u.email, u.role as "role: UserRole", u.is_active,
            p.id as "profile_id?", p.first_name, p.last_name, p.avatar_url, p.bio, p.address,
            p.created_at as "p_created_at?", p.updated_at as "p_updated_at?"
        FROM users u
        LEFT JOIN profiles p ON u.id = p.user_id
        LIMIT 1
        "#
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(row) = user_row {
        let profile = if let (Some(id), Some(created_at), Some(updated_at)) = (row.profile_id, row.p_created_at, row.p_updated_at) {
            Some(Profile {
                id,
                user_id: row.user_id,
                first_name: row.first_name,
                last_name: row.last_name,
                avatar_url: row.avatar_url,
                bio: row.bio,
                address: row.address,
                created_at,
                updated_at,
            })
        } else {
            None
        };

        Ok(Json(UserWithProfile {
            id: row.user_id,
            email: row.email,
            role: row.role,
            is_active: row.is_active,
            profile,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Utilisateur non trouvé".to_string()))
    }
}

pub async fn upsert_profile(
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateProfileInput>,
) -> Result<Json<Profile>, (StatusCode, String)> {
    // Simuler un user_id (on prend le premier user)
    let user_id = sqlx::query_scalar!("SELECT id FROM users LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let profile = sqlx::query_as!(
        Profile,
        r#"
        INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio, address)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id) DO UPDATE SET
            first_name = EXCLUDED.first_name,
            last_name = EXCLUDED.last_name,
            avatar_url = EXCLUDED.avatar_url,
            bio = EXCLUDED.bio,
            address = EXCLUDED.address,
            updated_at = NOW()
        RETURNING id, user_id, first_name, last_name, avatar_url, bio, address, created_at, updated_at
        "#,
        user_id,
        payload.first_name,
        payload.last_name,
        payload.avatar_url,
        payload.bio,
        payload.address
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(profile))
}