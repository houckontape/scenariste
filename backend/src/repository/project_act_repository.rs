use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project_act::{ProjectAct, CreateProjectActInput, UpdateProjectActInput};

pub struct ProjectActRepository;

impl ProjectActRepository {
    pub async fn list_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<Vec<ProjectAct>, sqlx::Error> {
        sqlx::query_as!(
            ProjectAct,
            r#"
            SELECT id, project_id, title, position, description, created_at, updated_at
            FROM project_acts
            WHERE project_id = $1
            ORDER BY position ASC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<ProjectAct>, sqlx::Error> {
        sqlx::query_as!(
            ProjectAct,
            r#"
            SELECT id, project_id, title, position, description, created_at, updated_at
            FROM project_acts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        input: CreateProjectActInput,
    ) -> Result<ProjectAct, sqlx::Error> {
        sqlx::query_as!(
            ProjectAct,
            r#"
            INSERT INTO project_acts (project_id, title, position, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id, project_id, title, position, description, created_at, updated_at
            "#,
            input.project_id,
            input.title,
            input.position,
            input.description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        input: UpdateProjectActInput,
    ) -> Result<ProjectAct, sqlx::Error> {
        sqlx::query_as!(
            ProjectAct,
            r#"
            UPDATE project_acts
            SET 
                title = COALESCE($1, title),
                position = COALESCE($2, position),
                description = COALESCE($3, description),
                updated_at = NOW()
            WHERE id = $4
            RETURNING id, project_id, title, position, description, created_at, updated_at
            "#,
            input.title,
            input.position,
            input.description,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM project_acts WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
