use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::brainstorming::{CreateBrainstormingNoteInput, UpdateBrainstormingNoteInput};
use crate::service::brainstorming_service::{BrainstormingService, BrainstormingServiceError};

pub async fn list_notes(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match BrainstormingService::list_notes(&pool, project_id, claims.sub).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn get_note(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match BrainstormingService::get_note(&pool, note_id, claims.sub).await {
        Ok(note) => Ok(Json(note)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn create_note(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateBrainstormingNoteInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match BrainstormingService::create_note(&pool, claims.sub, payload).await {
        Ok(note) => Ok((StatusCode::CREATED, Json(note))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_note(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(note_id): Path<Uuid>,
    Json(payload): Json<UpdateBrainstormingNoteInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match BrainstormingService::update_note(&pool, note_id, claims.sub, payload).await {
        Ok(note) => Ok(Json(note)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_note(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match BrainstormingService::delete_note(&pool, note_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: BrainstormingServiceError) -> (StatusCode, String) {
    match err {
        BrainstormingServiceError::NotFound => (StatusCode::NOT_FOUND, "Note non trouvée".to_string()),
        BrainstormingServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        BrainstormingServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
