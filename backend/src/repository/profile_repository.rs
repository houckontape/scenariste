use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct ProfileRepository;

impl ProfileRepository {
    pub async fn create_default_profile(
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio, address)
            VALUES ($1, NULL, NULL, NULL, NULL, NULL)
            "#,
            user_id
        )
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}