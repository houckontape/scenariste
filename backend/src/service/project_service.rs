use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project::{Project, ProjectWithRole, CreateProjectInput, UpdateProjectInput, MemberRole};
use crate::repository::project_repository::ProjectRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Projet non trouvé ou accès refusé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
}

pub struct ProjectService;

impl ProjectService {
    pub async fn list_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<ProjectWithRole>, ProjectServiceError> {
        ProjectRepository::list_for_user(pool, user_id)
            .await
            .map_err(ProjectServiceError::from)
    }

    pub async fn get_project(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<ProjectWithRole, ProjectServiceError> {
        let project = ProjectRepository::find_by_id(pool, project_id, user_id)
            .await?;
        
        project.ok_or(ProjectServiceError::NotFound)
    }

    pub async fn create_project(pool: &PgPool, user_id: Uuid, input: CreateProjectInput) -> Result<Project, ProjectServiceError> {
        ProjectRepository::create(pool, user_id, input)
            .await
            .map_err(ProjectServiceError::from)
    }

    pub async fn update_project(pool: &PgPool, project_id: Uuid, user_id: Uuid, input: UpdateProjectInput) -> Result<Project, ProjectServiceError> {
        let project_with_role = ProjectRepository::find_by_id(pool, project_id, user_id)
            .await?
            .ok_or(ProjectServiceError::NotFound)?;
            
        if project_with_role.user_role == MemberRole::Read {
            return Err(ProjectServiceError::Unauthorized);
        }
        
        ProjectRepository::update(pool, project_id, input)
            .await
            .map_err(ProjectServiceError::from)
    }

    pub async fn delete_project(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<(), ProjectServiceError> {
        let deleted = ProjectRepository::delete(pool, project_id, user_id)
            .await?;
        
        if deleted {
            Ok(())
        } else {
            // Si rien n'a été supprimé, c'est soit que le projet n'existe pas,
            // soit que l'utilisateur n'est pas 'owner'.
            Err(ProjectServiceError::NotFound)
        }
    }
}
