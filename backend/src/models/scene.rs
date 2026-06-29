use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "scene_setting", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SceneSetting {
    Int,
    Ext,
    IntExt,
}

impl SceneSetting {
    pub fn all() -> Vec<Self> {
        vec![Self::Int, Self::Ext, Self::IntExt]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "scene_time_of_day", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SceneTimeOfDay {
    Day,
    Night,
    Morning,
    Evening,
    Dawn,
    Dusk,
}

impl SceneTimeOfDay {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Day,
            Self::Night,
            Self::Morning,
            Self::Evening,
            Self::Dawn,
            Self::Dusk,
        ]
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Scene {
    pub id: Uuid,
    pub act_id: Uuid,
    pub project_id: Uuid,
    pub position: i32,
    pub setting: SceneSetting,
    pub location: String,
    pub time_of_day: SceneTimeOfDay,
    pub content: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSceneInput {
    pub act_id: Uuid,
    pub project_id: Uuid,
    pub position: i32,
    pub setting: SceneSetting,
    pub location: String,
    pub time_of_day: SceneTimeOfDay,
    pub content: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSceneInput {
    pub act_id: Option<Uuid>,
    pub position: Option<i32>,
    pub setting: Option<SceneSetting>,
    pub location: Option<String>,
    pub time_of_day: Option<SceneTimeOfDay>,
    pub content: Option<String>,
    pub note: Option<String>,
}
