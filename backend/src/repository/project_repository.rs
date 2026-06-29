use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project::{Project, ProjectWithRole, CreateProjectInput, UpdateProjectInput, ProjectType, MemberRole};
use slug::slugify;

pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn list_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<ProjectWithRole>, sqlx::Error> {
        let projects = sqlx::query_as!(
            ProjectWithRole,
            r#"
            SELECT 
                p.id, p.title, p.slug, p.description, p.p_type as "p_type: ProjectType", 
                p.is_private, p.created_at, p.updated_at,
                pm.role as "user_role: MemberRole"
            FROM projects p
            JOIN project_members pm ON p.id = pm.project_id
            WHERE pm.user_id = $1
            ORDER BY p.updated_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    pub async fn find_by_id(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Option<ProjectWithRole>, sqlx::Error> {
        let project = sqlx::query_as!(
            ProjectWithRole,
            r#"
            SELECT 
                p.id, p.title, p.slug, p.description, p.p_type as "p_type: ProjectType", 
                p.is_private, p.created_at, p.updated_at,
                pm.role as "user_role: MemberRole"
            FROM projects p
            JOIN project_members pm ON p.id = pm.project_id
            WHERE p.id = $1 AND pm.user_id = $2
            "#,
            project_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(project)
    }

    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateProjectInput) -> Result<Project, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let slug = slugify(&input.title);

        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (title, slug, description, p_type, is_private)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, slug, description, p_type as "p_type: ProjectType", is_private, created_at, updated_at
            "#,
            input.title,
            slug,
            input.description,
            input.p_type as ProjectType,
            input.is_private
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO project_members (project_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, NOW())
            "#,
            project.id,
            user_id,
            MemberRole::Owner as MemberRole
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(project)
    }

    pub async fn update(pool: &PgPool, project_id: Uuid, input: UpdateProjectInput) -> Result<Project, sqlx::Error> {
        let slug = input.title.as_ref().map(|t| slugify(t));

        let project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET 
                title = COALESCE($2, title),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description),
                p_type = COALESCE($5, p_type),
                is_private = COALESCE($6, is_private),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, title, slug, description, p_type as "p_type: ProjectType", is_private, created_at, updated_at
            "#,
            project_id,
            input.title,
            slug,
            input.description,
            input.p_type as Option<ProjectType>,
            input.is_private
        )
        .fetch_one(pool)
        .await?;

        Ok(project)
    }

    pub async fn delete(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        // Seul le propriétaire peut supprimer le projet
        let result = sqlx::query!(
            r#"
            DELETE FROM projects 
            WHERE id = $1 AND id IN (
                SELECT project_id FROM project_members 
                WHERE project_id = $1 AND user_id = $2 AND role = 'owner'
            )
            "#,
            project_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
