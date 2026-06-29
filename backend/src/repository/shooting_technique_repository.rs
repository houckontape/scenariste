use sqlx::PgPool;
use crate::models::shooting_technique::ShootingTechnique;

pub struct ShootingTechniqueRepository;

impl ShootingTechniqueRepository {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<ShootingTechnique>, sqlx::Error> {
        let techniques = sqlx::query_as!(
            ShootingTechnique,
            r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM shooting_techniques
            ORDER BY name ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(techniques)
    }
}
