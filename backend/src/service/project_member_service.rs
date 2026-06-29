use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project::MemberRole;
use crate::models::project_member::{ProjectMemberDetails, AddMemberInput, UpdateMemberRoleInput};
use crate::repository::project_member_repository::ProjectMemberRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectMemberServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Membre non trouvé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
    #[error("Utilisateur introuvable avec cet email")]
    UserNotFound,
    #[error("L'utilisateur est déjà membre du projet")]
    AlreadyMember,
    #[error("Impossible de supprimer le dernier propriétaire")]
    LastOwner,
}

pub struct ProjectMemberService;

impl ProjectMemberService {
    pub async fn list_members(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<ProjectMemberDetails>, ProjectMemberServiceError> {
        ProjectMemberRepository::list_by_project(pool, project_id, user_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ProjectMemberServiceError::Unauthorized,
                _ => ProjectMemberServiceError::DatabaseError(e),
            })
    }

    pub async fn add_member(
        pool: &PgPool,
        project_id: Uuid,
        requester_id: Uuid,
        input: AddMemberInput,
    ) -> Result<ProjectMemberDetails, ProjectMemberServiceError> {
        // 1. Vérifier que le demandeur est Owner
        let role = ProjectMemberRepository::get_user_role(pool, project_id, requester_id).await?;
        if role != Some(MemberRole::Owner) {
            return Err(ProjectMemberServiceError::Unauthorized);
        }

        // 2. Ajouter le membre
        ProjectMemberRepository::add_member(pool, project_id, &input.email, input.role)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ProjectMemberServiceError::UserNotFound,
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => ProjectMemberServiceError::AlreadyMember,
                _ => ProjectMemberServiceError::DatabaseError(e),
            })
    }

    pub async fn update_member_role(
        pool: &PgPool,
        project_id: Uuid,
        requester_id: Uuid,
        target_user_id: Uuid,
        input: UpdateMemberRoleInput,
    ) -> Result<ProjectMemberDetails, ProjectMemberServiceError> {
        // 1. Vérifier que le demandeur est Owner
        let role = ProjectMemberRepository::get_user_role(pool, project_id, requester_id).await?;
        if role != Some(MemberRole::Owner) {
            return Err(ProjectMemberServiceError::Unauthorized);
        }

        // 2. Modifier le rôle
        ProjectMemberRepository::update_role(pool, project_id, target_user_id, input.role)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ProjectMemberServiceError::NotFound,
                _ => ProjectMemberServiceError::DatabaseError(e),
            })
    }

    pub async fn remove_member(
        pool: &PgPool,
        project_id: Uuid,
        requester_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), ProjectMemberServiceError> {
        // 1. Vérifier que le demandeur est Owner
        let role = ProjectMemberRepository::get_user_role(pool, project_id, requester_id).await?;
        if role != Some(MemberRole::Owner) {
            return Err(ProjectMemberServiceError::Unauthorized);
        }

        // 2. Vérifier s'il reste d'autres owners si on supprime un owner
        let target_role = ProjectMemberRepository::get_user_role(pool, project_id, target_user_id).await?;
        if target_role == Some(MemberRole::Owner) {
            let owners_count = sqlx::query_scalar!(
                "SELECT count(*) FROM project_members WHERE project_id = $1 AND role = 'owner'",
                project_id
            )
            .fetch_one(pool)
            .await
            .map_err(ProjectMemberServiceError::DatabaseError)?;

            if owners_count.unwrap_or(0) <= 1 {
                return Err(ProjectMemberServiceError::LastOwner);
            }
        }

        // 3. Supprimer
        let deleted = ProjectMemberRepository::remove_member(pool, project_id, target_user_id).await?;
        if deleted {
            Ok(())
        } else {
            Err(ProjectMemberServiceError::NotFound)
        }
    }
}
