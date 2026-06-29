use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::project_synopsis::UpdateProjectSynopsisInput;
use crate::service::project_synopsis_service::{ProjectSynopsisService, ProjectSynopsisServiceError};

pub async fn get_synopsis(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectSynopsisService::get_synopsis(&pool, project_id, claims.sub).await {
        Ok(synopsis) => Ok(Json(synopsis)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn upsert_synopsis(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<UpdateProjectSynopsisInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectSynopsisService::upsert_synopsis(&pool, project_id, claims.sub, payload).await {
        Ok(synopsis) => Ok(Json(synopsis)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_synopsis(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectSynopsisService::delete_synopsis(&pool, project_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: ProjectSynopsisServiceError) -> (StatusCode, String) {
    match err {
        ProjectSynopsisServiceError::NotFound => (StatusCode::NOT_FOUND, "Synopsis non trouvé".to_string()),
        ProjectSynopsisServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        ProjectSynopsisServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
