use sqlx::{Postgres, Transaction};
use crate::models::user::UserRole;
use uuid::Uuid; // <-- Ajoute l'import de uuid

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        tx: &mut Transaction<'_, Postgres>,
        email: &str,
        password_hash: &str,
    ) -> Result<Uuid, sqlx::Error> { // <-- Change i32 par Uuid
        let row = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, role, is_active)
            VALUES ($1, $2, $3::user_role, true)
            RETURNING id
            "#,
            email.trim().to_lowercase(),
            password_hash,
            UserRole::FreeUser as UserRole
        )
            .fetch_one(&mut **tx)
            .await?;

        Ok(row.id) // SQLx va automatiquement mapper le type UUID de Postgres vers uuid::Uuid en Rust
    }
}