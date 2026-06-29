use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project::MemberRole;
use crate::models::project_member::{ProjectMemberDetails};

pub struct ProjectMemberRepository;

impl ProjectMemberRepository {
    pub async fn list_by_project(
        pool: &PgPool,
        project_id: Uuid,
        requester_id: Uuid,
    ) -> Result<Vec<ProjectMemberDetails>, sqlx::Error> {
        // Vérifier d'abord si le demandeur est membre du projet
        let is_member = sqlx::query!(
            "SELECT 1 as \"exists!\" FROM project_members WHERE project_id = $1 AND user_id = $2",
            project_id,
            requester_id
        )
        .fetch_optional(pool)
        .await?;

        if is_member.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let members = sqlx::query_as!(
            ProjectMemberDetails,
            r#"
            SELECT 
                pm.project_id, pm.user_id, u.email, pm.role as "role: MemberRole", 
                pm.invited_at, pm.joined_at, p.first_name, p.last_name
            FROM project_members pm
            JOIN users u ON pm.user_id = u.id
            LEFT JOIN profiles p ON u.id = p.user_id
            WHERE pm.project_id = $1
            ORDER BY pm.invited_at ASC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    pub async fn add_member(
        pool: &PgPool,
        project_id: Uuid,
        email: &str,
        role: MemberRole,
    ) -> Result<ProjectMemberDetails, sqlx::Error> {
        // Trouver l'utilisateur par email
        let user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            email.trim().to_lowercase()
        )
        .fetch_optional(pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let member = sqlx::query_as!(
            ProjectMemberDetails,
            r#"
            WITH inserted AS (
                INSERT INTO project_members (project_id, user_id, role)
                VALUES ($1, $2, $3::member_role)
                RETURNING project_id, user_id, role, invited_at, joined_at
            )
            SELECT 
                i.project_id, i.user_id, u.email, i.role as "role: MemberRole", 
                i.invited_at, i.joined_at, prof.first_name, prof.last_name
            FROM inserted i
            JOIN users u ON i.user_id = u.id
            LEFT JOIN profiles prof ON u.id = prof.user_id
            "#,
            project_id,
            user.id,
            role as MemberRole
        )
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn update_role(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        new_role: MemberRole,
    ) -> Result<ProjectMemberDetails, sqlx::Error> {
        let member = sqlx::query_as!(
            ProjectMemberDetails,
            r#"
            WITH updated AS (
                UPDATE project_members
                SET role = $1::member_role
                WHERE project_id = $2 AND user_id = $3
                RETURNING project_id, user_id, role, invited_at, joined_at
            )
            SELECT 
                u_row.project_id, u_row.user_id, u.email, u_row.role as "role: MemberRole", 
                u_row.invited_at, u_row.joined_at, prof.first_name, prof.last_name
            FROM updated u_row
            JOIN users u ON u_row.user_id = u.id
            LEFT JOIN profiles prof ON u.id = prof.user_id
            "#,
            new_role as MemberRole,
            project_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn remove_member(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM project_members WHERE project_id = $1 AND user_id = $2",
            project_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_user_role(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<MemberRole>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT role as "role: MemberRole" FROM project_members WHERE project_id = $1 AND user_id = $2"#,
            project_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.role))
    }
}
