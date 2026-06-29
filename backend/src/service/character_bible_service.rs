use sqlx::PgPool;
use uuid::Uuid;
use crate::models::character_bible::{CharacterBible, CreateCharacterInput, UpdateCharacterInput};
use crate::repository::character_bible_repository::CharacterBibleRepository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CharacterBibleServiceError {
    #[error("Personnage non trouvé")]
    NotFound,
    #[error("Accès non autorisé")]
    Unauthorized,
    #[error("Erreur de base de données : {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct CharacterBibleService;

impl CharacterBibleService {
    pub async fn list_characters(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<CharacterBible>, CharacterBibleServiceError> {
        CharacterBibleRepository::list_by_project(pool, project_id, user_id)
            .await
            .map_err(CharacterBibleServiceError::DatabaseError)
    }

    pub async fn get_character(pool: &PgPool, character_id: Uuid, user_id: Uuid) -> Result<CharacterBible, CharacterBibleServiceError> {
        let character = CharacterBibleRepository::find_by_id(pool, character_id, user_id)
            .await
            .map_err(CharacterBibleServiceError::DatabaseError)?;

        character.ok_or(CharacterBibleServiceError::NotFound)
    }

    pub async fn create_character(pool: &PgPool, user_id: Uuid, input: CreateCharacterInput) -> Result<CharacterBible, CharacterBibleServiceError> {
        CharacterBibleRepository::create(pool, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => CharacterBibleServiceError::Unauthorized,
                _ => CharacterBibleServiceError::DatabaseError(e),
            })
    }

    pub async fn update_character(pool: &PgPool, character_id: Uuid, user_id: Uuid, input: UpdateCharacterInput) -> Result<CharacterBible, CharacterBibleServiceError> {
        CharacterBibleRepository::update(pool, character_id, user_id, input)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => CharacterBibleServiceError::NotFound,
                _ => CharacterBibleServiceError::DatabaseError(e),
            })
    }

    pub async fn delete_character(pool: &PgPool, character_id: Uuid, user_id: Uuid) -> Result<(), CharacterBibleServiceError> {
        let deleted = CharacterBibleRepository::delete(pool, character_id, user_id)
            .await
            .map_err(CharacterBibleServiceError::DatabaseError)?;

        if deleted {
            Ok(())
        } else {
            Err(CharacterBibleServiceError::NotFound)
        }
    }
}
