use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project_synopsis::{ProjectSynopsis, UpdateProjectSynopsisInput};

pub struct ProjectSynopsisRepository;

impl ProjectSynopsisRepository {
    pub async fn get_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<Option<ProjectSynopsis>, sqlx::Error> {
        sqlx::query_as!(
            ProjectSynopsis,
            r#"
            SELECT id, project_id, logline, summary_short, summary_long, updated_at
            FROM project_synopses
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn upsert(
        pool: &PgPool,
        project_id: Uuid,
        input: UpdateProjectSynopsisInput,
    ) -> Result<ProjectSynopsis, sqlx::Error> {
        sqlx::query_as!(
            ProjectSynopsis,
            r#"
            INSERT INTO project_synopses (project_id, logline, summary_short, summary_long)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (project_id) DO UPDATE SET
                logline = EXCLUDED.logline,
                summary_short = EXCLUDED.summary_short,
                summary_long = EXCLUDED.summary_long,
                updated_at = NOW()
            RETURNING id, project_id, logline, summary_short, summary_long, updated_at
            "#,
            project_id,
            input.logline,
            input.summary_short,
            input.summary_long
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM project_synopses WHERE project_id = $1",
            project_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
