use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::character_bible::{CreateCharacterInput, UpdateCharacterInput};
use crate::service::character_bible_service::{CharacterBibleService, CharacterBibleServiceError};

pub async fn list_characters(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match CharacterBibleService::list_characters(&pool, project_id, claims.sub).await {
        Ok(characters) => Ok(Json(characters)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn get_character(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(character_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match CharacterBibleService::get_character(&pool, character_id, claims.sub).await {
        Ok(character) => Ok(Json(character)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn create_character(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateCharacterInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match CharacterBibleService::create_character(&pool, claims.sub, payload).await {
        Ok(character) => Ok((StatusCode::CREATED, Json(character))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_character(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(character_id): Path<Uuid>,
    Json(payload): Json<UpdateCharacterInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match CharacterBibleService::update_character(&pool, character_id, claims.sub, payload).await {
        Ok(character) => Ok(Json(character)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_character(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(character_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match CharacterBibleService::delete_character(&pool, character_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: CharacterBibleServiceError) -> (StatusCode, String) {
    match err {
        CharacterBibleServiceError::NotFound => (StatusCode::NOT_FOUND, "Personnage non trouvé".to_string()),
        CharacterBibleServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        CharacterBibleServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
