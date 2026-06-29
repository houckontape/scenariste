use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::project_act::{CreateProjectActInput, UpdateProjectActInput};
use crate::service::project_act_service::{ProjectActService, ProjectActServiceError};

pub async fn list_acts(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectActService::list_acts(&pool, project_id, claims.sub).await {
        Ok(acts) => Ok(Json(acts)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn get_act(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(act_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectActService::get_act(&pool, act_id, claims.sub).await {
        Ok(act) => Ok(Json(act)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn create_act(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateProjectActInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectActService::create_act(&pool, claims.sub, payload).await {
        Ok(act) => Ok((StatusCode::CREATED, Json(act))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_act(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(act_id): Path<Uuid>,
    Json(payload): Json<UpdateProjectActInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectActService::update_act(&pool, act_id, claims.sub, payload).await {
        Ok(act) => Ok(Json(act)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_act(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(act_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectActService::delete_act(&pool, act_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: ProjectActServiceError) -> (StatusCode, String) {
    match err {
        ProjectActServiceError::NotFound => (StatusCode::NOT_FOUND, "Acte non trouvé".to_string()),
        ProjectActServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        ProjectActServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
