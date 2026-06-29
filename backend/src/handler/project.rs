use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::project::{CreateProjectInput, UpdateProjectInput};
use crate::service::project_service::{ProjectService, ProjectServiceError};

pub async fn list_projects(
    State(pool): State<PgPool>,
    claims: Claims,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectService::list_projects(&pool, claims.sub).await {
        Ok(projects) => Ok(Json(projects)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn get_project(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectService::get_project(&pool, project_id, claims.sub).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn create_project(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateProjectInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectService::create_project(&pool, claims.sub, payload).await {
        Ok(project) => Ok((StatusCode::CREATED, Json(project))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_project(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<UpdateProjectInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectService::update_project(&pool, project_id, claims.sub, payload).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_project(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectService::delete_project(&pool, project_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: ProjectServiceError) -> (StatusCode, String) {
    match err {
        ProjectServiceError::NotFound => (StatusCode::NOT_FOUND, "Projet non trouvé".to_string()),
        ProjectServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        ProjectServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
