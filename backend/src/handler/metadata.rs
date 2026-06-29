use axum::{response::IntoResponse, Json};
use crate::models::scene::{SceneSetting, SceneTimeOfDay};

pub async fn get_scene_settings() -> impl IntoResponse {
    Json(SceneSetting::all())
}

pub async fn get_scene_times_of_day() -> impl IntoResponse {
    Json(SceneTimeOfDay::all())
}
