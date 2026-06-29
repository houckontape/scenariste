use sqlx::PgPool;
use uuid::Uuid;
use crate::models::brainstorming::{BrainstormingNote, CreateBrainstormingNoteInput, UpdateBrainstormingNoteInput};
use crate::repository::brainstorming_repository::BrainstormingRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainstormingServiceError {
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Note non trouvée ou accès refusé")]
    NotFound,
    #[error("Action non autorisée")]
    Unauthorized,
}

pub struct BrainstormingService;

impl BrainstormingService {
    pub async fn list_notes(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<BrainstormingNote>, BrainstormingServiceError> {
        BrainstormingRepository::list_by_project(pool, project_id, user_id)
            .await
            .map_err(BrainstormingServiceError::from)
    }

    pub async fn get_note(pool: &PgPool, note_id: Uuid, user_id: Uuid) -> Result<BrainstormingNote, BrainstormingServiceError> {
        let note = BrainstormingRepository::find_by_id(pool, note_id, user_id)
            .await?;
        
        note.ok_or(BrainstormingServiceError::NotFound)
    }

    pub async fn create_note(pool: &PgPool, user_id: Uuid, input: CreateBrainstormingNoteInput) -> Result<BrainstormingNote, BrainstormingServiceError> {
        BrainstormingRepository::create(pool, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => BrainstormingServiceError::Unauthorized,
                _ => BrainstormingServiceError::DatabaseError(e),
            })
    }

    pub async fn update_note(pool: &PgPool, note_id: Uuid, user_id: Uuid, input: UpdateBrainstormingNoteInput) -> Result<BrainstormingNote, BrainstormingServiceError> {
        BrainstormingRepository::update(pool, note_id, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => BrainstormingServiceError::NotFound,
                _ => BrainstormingServiceError::DatabaseError(e),
            })
    }

    pub async fn delete_note(pool: &PgPool, note_id: Uuid, user_id: Uuid) -> Result<(), BrainstormingServiceError> {
        let deleted = BrainstormingRepository::delete(pool, note_id, user_id)
            .await?;
        
        if deleted {
            Ok(())
        } else {
            Err(BrainstormingServiceError::NotFound)
        }
    }
}
