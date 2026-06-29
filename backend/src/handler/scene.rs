use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::scene::{CreateSceneInput, UpdateSceneInput};
use crate::service::scene_service::{SceneService, SceneServiceError};

pub async fn list_scenes(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match SceneService::list_scenes(&pool, project_id, claims.sub).await {
        Ok(scenes) => Ok(Json(scenes)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn get_scene(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(scene_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match SceneService::get_scene(&pool, scene_id, claims.sub).await {
        Ok(scene) => Ok(Json(scene)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn create_scene(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateSceneInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match SceneService::create_scene(&pool, claims.sub, payload).await {
        Ok(scene) => Ok((StatusCode::CREATED, Json(scene))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_scene(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(scene_id): Path<Uuid>,
    Json(payload): Json<UpdateSceneInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match SceneService::update_scene(&pool, scene_id, claims.sub, payload).await {
        Ok(scene) => Ok(Json(scene)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn delete_scene(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(scene_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match SceneService::delete_scene(&pool, scene_id, claims.sub).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: SceneServiceError) -> (StatusCode, String) {
    match err {
        SceneServiceError::NotFound => (StatusCode::NOT_FOUND, "Scène non trouvée".to_string()),
        SceneServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        SceneServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
