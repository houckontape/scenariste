use sqlx::PgPool;
use uuid::Uuid;
use crate::models::scene::{Scene, CreateSceneInput, UpdateSceneInput, SceneSetting, SceneTimeOfDay};

pub struct SceneRepository;

impl SceneRepository {
    pub async fn list_by_project(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<Scene>, sqlx::Error> {
        let scenes = sqlx::query_as!(
            Scene,
            r#"
            SELECT 
                s.id, s.act_id, s.project_id, s.position, 
                s.setting as "setting: SceneSetting", 
                s.location, 
                s.time_of_day as "time_of_day: SceneTimeOfDay", 
                s.content, s.note, s.shooting_technique_id, s.created_at, s.updated_at
            FROM scenes s
            JOIN project_members pm ON s.project_id = pm.project_id
            WHERE s.project_id = $1 AND pm.user_id = $2
            ORDER BY s.position ASC
            "#,
            project_id,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(scenes)
    }

    pub async fn find_by_id(pool: &PgPool, scene_id: Uuid, user_id: Uuid) -> Result<Option<Scene>, sqlx::Error> {
        let scene = sqlx::query_as!(
            Scene,
            r#"
            SELECT 
                s.id, s.act_id, s.project_id, s.position, 
                s.setting as "setting: SceneSetting", 
                s.location, 
                s.time_of_day as "time_of_day: SceneTimeOfDay", 
                s.content, s.note, s.shooting_technique_id, s.created_at, s.updated_at
            FROM scenes s
            JOIN project_members pm ON s.project_id = pm.project_id
            WHERE s.id = $1 AND pm.user_id = $2
            "#,
            scene_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(scene)
    }

    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateSceneInput) -> Result<Scene, sqlx::Error> {
        // On vérifie d'abord que l'utilisateur a le droit d'écrire dans le projet
        // (rôle owner ou write)
        let has_access = sqlx::query!(
            r#"
            SELECT 1 as "access!" FROM project_members 
            WHERE project_id = $1 AND user_id = $2 AND role IN ('owner', 'write')
            "#,
            input.project_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if has_access.is_none() {
            return Err(sqlx::Error::RowNotFound); // Ou une erreur personnalisée plus tard
        }

        let scene = sqlx::query_as!(
            Scene,
            r#"
            INSERT INTO scenes (act_id, project_id, position, setting, location, time_of_day, content, note, shooting_technique_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, act_id, project_id, position, 
                setting as "setting: SceneSetting", 
                location, 
                time_of_day as "time_of_day: SceneTimeOfDay", 
                content, note, shooting_technique_id, created_at, updated_at
            "#,
            input.act_id,
            input.project_id,
            input.position,
            input.setting as SceneSetting,
            input.location,
            input.time_of_day as SceneTimeOfDay,
            input.content.unwrap_or_default(),
            input.note,
            input.shooting_technique_id
        )
        .fetch_one(pool)
        .await?;

        Ok(scene)
    }

    pub async fn update(pool: &PgPool, scene_id: Uuid, user_id: Uuid, input: UpdateSceneInput) -> Result<Scene, sqlx::Error> {
        let scene = sqlx::query_as!(
            Scene,
            r#"
            UPDATE scenes
            SET 
                act_id = COALESCE($1, act_id),
                position = COALESCE($2, position),
                setting = COALESCE($3, setting),
                location = COALESCE($4, location),
                time_of_day = COALESCE($5, time_of_day),
                content = COALESCE($6, content),
                note = COALESCE($7, note),
                shooting_technique_id = COALESCE($8, shooting_technique_id),
                updated_at = NOW()
            WHERE id = $9 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $10 AND role IN ('owner', 'write')
            )
            RETURNING 
                id, act_id, project_id, position, 
                setting as "setting: SceneSetting", 
                location, 
                time_of_day as "time_of_day: SceneTimeOfDay", 
                content, note, shooting_technique_id, created_at, updated_at
            "#,
            input.act_id,
            input.position,
            input.setting as Option<SceneSetting>,
            input.location,
            input.time_of_day as Option<SceneTimeOfDay>,
            input.content,
            input.note,
            input.shooting_technique_id,
            scene_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(scene)
    }

    pub async fn delete(pool: &PgPool, scene_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM scenes 
            WHERE id = $1 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $2 AND role IN ('owner', 'write')
            )
            "#,
            scene_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
