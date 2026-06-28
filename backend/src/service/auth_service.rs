use uuid::Uuid; // <-- Assure-toi que cet import est présent
use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2, PasswordVerifier};
use sqlx::PgPool;
use crate::repository::user_repository::UserRepository;
use crate::repository::profile_repository::ProfileRepository;
use crate::models::auth::{Claims, AuthResponse, UserSummary};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::models::user::{User, UserRole};
use argon2::PasswordHash;

pub enum AuthServiceError {
    PasswordTooShort,
    EmailConflict,
    EntropyError,
    DatabaseError(String),
    InvalidCredentials,
    TokenError,
}

pub struct AuthService;

impl AuthService {
    pub async fn register_user(
        pool: &PgPool,
        email: &str,
        raw_password: &str,
    ) -> Result<Uuid, AuthServiceError> { // <-- Doit être Uuid, pas i32
        // 1. Validation métier
        if raw_password.len() < 8 {
            return Err(AuthServiceError::PasswordTooShort);
        }

        // 2. Chiffrement du mot de passe
        let mut salt_bytes = [0u8; 16];
        argon2::password_hash::rand_core::RngCore::try_fill_bytes(
            &mut argon2::password_hash::rand_core::OsRng,
            &mut salt_bytes
        ).map_err(|_| AuthServiceError::EntropyError)?;

        let salt = SaltString::encode_b64(&salt_bytes)
            .map_err(|_| AuthServiceError::EntropyError)?;

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|_| AuthServiceError::EntropyError)?
            .to_string();

        // 3. Gestion de la Transaction
        let mut tx = pool.begin().await
            .map_err(|e| AuthServiceError::DatabaseError(e.to_string()))?;

        // Étape A: Création de l'user (user_id récupère ici le Uuid du UserRepository)
        let user_id: Uuid = UserRepository::create_user(&mut tx, email, &password_hash)
            .await
            .map_err(|e| {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                        return AuthServiceError::EmailConflict;
                    }
                }
                AuthServiceError::DatabaseError(e.to_string())
            })?;

        // Étape B: Création du profil lié (on passe le Uuid)
        ProfileRepository::create_default_profile(&mut tx, user_id)
            .await
            .map_err(|e| AuthServiceError::DatabaseError(e.to_string()))?;

        // Étape C: Commit
        tx.commit().await
            .map_err(|e| AuthServiceError::DatabaseError(e.to_string()))?;

        Ok(user_id) // <-- Retourne le Uuid
    }

    pub async fn login(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, AuthServiceError> {
        // 1. Récupérer l'utilisateur
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, role as "role: UserRole", is_active, created_at, updated_at FROM users WHERE email = $1"#,
            email.trim().to_lowercase()
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AuthServiceError::DatabaseError(e.to_string()))?
        .ok_or(AuthServiceError::InvalidCredentials)?;

        // 2. Vérifier le mot de passe
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| AuthServiceError::EntropyError)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;

        // 3. Générer le JWT
        let secret = std::env::var("JWT_SECRET").map_err(|_| AuthServiceError::TokenError)?;
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id,
            exp: expiration as usize,
            role: user.role,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| AuthServiceError::TokenError)?;

        Ok(AuthResponse {
            token,
            user: UserSummary {
                id: user.id,
                email: user.email,
                role: user.role,
            },
        })
    }
}