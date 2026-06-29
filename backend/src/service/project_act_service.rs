use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project_act::{ProjectAct, CreateProjectActInput, UpdateProjectActInput};
use crate::models::project::MemberRole;
use crate::repository::project_act_repository::ProjectActRepository;
use crate::repository::project_member_repository::ProjectMemberRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectActServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Acte non trouvé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
}

pub struct ProjectActService;

impl ProjectActService {
    pub async fn list_acts(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<ProjectAct>, ProjectActServiceError> {
        // Vérifier si l'utilisateur est membre du projet
        let role = ProjectMemberRepository::get_user_role(pool, project_id, user_id).await?;
        if role.is_none() {
            return Err(ProjectActServiceError::Unauthorized);
        }

        ProjectActRepository::list_by_project_id(pool, project_id)
            .await
            .map_err(ProjectActServiceError::DatabaseError)
    }

    pub async fn get_act(
        pool: &PgPool,
        act_id: Uuid,
        user_id: Uuid,
    ) -> Result<ProjectAct, ProjectActServiceError> {
        let act = ProjectActRepository::get_by_id(pool, act_id)
            .await?
            .ok_or(ProjectActServiceError::NotFound)?;

        // Vérifier si l'utilisateur est membre du projet
        let role = ProjectMemberRepository::get_user_role(pool, act.project_id, user_id).await?;
        if role.is_none() {
            return Err(ProjectActServiceError::Unauthorized);
        }

        Ok(act)
    }

    pub async fn create_act(
        pool: &PgPool,
        user_id: Uuid,
        input: CreateProjectActInput,
    ) -> Result<ProjectAct, ProjectActServiceError> {
        // Vérifier les droits d'écriture
        let role = ProjectMemberRepository::get_user_role(pool, input.project_id, user_id).await?;
        match role {
            Some(MemberRole::Owner) | Some(MemberRole::Write) => {
                ProjectActRepository::create(pool, input)
                    .await
                    .map_err(ProjectActServiceError::DatabaseError)
            }
            _ => Err(ProjectActServiceError::Unauthorized),
        }
    }

    pub async fn update_act(
        pool: &PgPool,
        act_id: Uuid,
        user_id: Uuid,
        input: UpdateProjectActInput,
    ) -> Result<ProjectAct, ProjectActServiceError> {
        let act = ProjectActRepository::get_by_id(pool, act_id)
            .await?
            .ok_or(ProjectActServiceError::NotFound)?;

        // Vérifier les droits d'écriture
        let role = ProjectMemberRepository::get_user_role(pool, act.project_id, user_id).await?;
        match role {
            Some(MemberRole::Owner) | Some(MemberRole::Write) => {
                ProjectActRepository::update(pool, act_id, input)
                    .await
                    .map_err(ProjectActServiceError::DatabaseError)
            }
            _ => Err(ProjectActServiceError::Unauthorized),
        }
    }

    pub async fn delete_act(
        pool: &PgPool,
        act_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ProjectActServiceError> {
        let act = ProjectActRepository::get_by_id(pool, act_id)
            .await?
            .ok_or(ProjectActServiceError::NotFound)?;

        // Vérifier les droits d'écriture
        let role = ProjectMemberRepository::get_user_role(pool, act.project_id, user_id).await?;
        match role {
            Some(MemberRole::Owner) | Some(MemberRole::Write) => {
                let deleted = ProjectActRepository::delete(pool, act_id).await?;
                if deleted {
                    Ok(())
                } else {
                    Err(ProjectActServiceError::NotFound)
                }
            }
            _ => Err(ProjectActServiceError::Unauthorized),
        }
    }
}
