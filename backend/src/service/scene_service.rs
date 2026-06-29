use sqlx::PgPool;
use uuid::Uuid;
use crate::models::scene::{Scene, CreateSceneInput, UpdateSceneInput};
use crate::repository::scene_repository::SceneRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Scène non trouvée ou accès refusé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
}

pub struct SceneService;

impl SceneService {
    pub async fn list_scenes(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<Scene>, SceneServiceError> {
        SceneRepository::list_by_project(pool, project_id, user_id)
            .await
            .map_err(SceneServiceError::from)
    }

    pub async fn get_scene(pool: &PgPool, scene_id: Uuid, user_id: Uuid) -> Result<Scene, SceneServiceError> {
        let scene = SceneRepository::find_by_id(pool, scene_id, user_id)
            .await?;
        
        scene.ok_or(SceneServiceError::NotFound)
    }

    pub async fn create_scene(pool: &PgPool, user_id: Uuid, input: CreateSceneInput) -> Result<Scene, SceneServiceError> {
        // Le repository vérifie déjà l'accès, mais on pourrait affiner ici si besoin
        SceneRepository::create(pool, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => SceneServiceError::Unauthorized,
                _ => SceneServiceError::DatabaseError(e),
            })
    }

    pub async fn update_scene(pool: &PgPool, scene_id: Uuid, user_id: Uuid, input: UpdateSceneInput) -> Result<Scene, SceneServiceError> {
        SceneRepository::update(pool, scene_id, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => SceneServiceError::Unauthorized,
                _ => SceneServiceError::DatabaseError(e),
            })
    }

    pub async fn delete_scene(pool: &PgPool, scene_id: Uuid, user_id: Uuid) -> Result<(), SceneServiceError> {
        let deleted = SceneRepository::delete(pool, scene_id, user_id)
            .await?;
        
        if deleted {
            Ok(())
        } else {
            Err(SceneServiceError::NotFound)
        }
    }
}
