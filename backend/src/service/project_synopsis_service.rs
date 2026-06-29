use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project_synopsis::{ProjectSynopsis, UpdateProjectSynopsisInput};
use crate::models::project::MemberRole;
use crate::repository::project_synopsis_repository::ProjectSynopsisRepository;
use crate::repository::project_member_repository::ProjectMemberRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectSynopsisServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Synopsis non trouvé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
}

pub struct ProjectSynopsisService;

impl ProjectSynopsisService {
    pub async fn get_synopsis(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<ProjectSynopsis, ProjectSynopsisServiceError> {
        // Vérifier si l'utilisateur est membre du projet
        let role = ProjectMemberRepository::get_user_role(pool, project_id, user_id).await?;
        if role.is_none() {
            return Err(ProjectSynopsisServiceError::Unauthorized);
        }

        ProjectSynopsisRepository::get_by_project_id(pool, project_id)
            .await?
            .ok_or(ProjectSynopsisServiceError::NotFound)
    }

    pub async fn upsert_synopsis(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        input: UpdateProjectSynopsisInput,
    ) -> Result<ProjectSynopsis, ProjectSynopsisServiceError> {
        // Vérifier si l'utilisateur est membre avec droits d'écriture
        let role = ProjectMemberRepository::get_user_role(pool, project_id, user_id).await?;
        match role {
            Some(MemberRole::Owner) | Some(MemberRole::Write) => {
                ProjectSynopsisRepository::upsert(pool, project_id, input)
                    .await
                    .map_err(ProjectSynopsisServiceError::DatabaseError)
            }
            _ => Err(ProjectSynopsisServiceError::Unauthorized),
        }
    }

    pub async fn delete_synopsis(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ProjectSynopsisServiceError> {
        // Vérifier si l'utilisateur est membre avec droits d'écriture
        let role = ProjectMemberRepository::get_user_role(pool, project_id, user_id).await?;
        match role {
            Some(MemberRole::Owner) | Some(MemberRole::Write) => {
                let deleted = ProjectSynopsisRepository::delete(pool, project_id).await?;
                if deleted {
                    Ok(())
                } else {
                    Err(ProjectSynopsisServiceError::NotFound)
                }
            }
            _ => Err(ProjectSynopsisServiceError::Unauthorized),
        }
    }
}
