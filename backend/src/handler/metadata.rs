use axum::{extract::State, response::IntoResponse, Json};
use sqlx::PgPool;
use crate::models::scene::{SceneSetting, SceneTimeOfDay};
use crate::repository::shooting_technique_repository::ShootingTechniqueRepository;

pub async fn get_scene_settings() -> impl IntoResponse {
    Json(SceneSetting::all())
}

pub async fn get_scene_times_of_day() -> impl IntoResponse {
    Json(SceneTimeOfDay::all())
}

pub async fn get_shooting_techniques(State(pool): State<PgPool>) -> impl IntoResponse {
    match ShootingTechniqueRepository::list_all(&pool).await {
        Ok(techniques) => Json(techniques).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
