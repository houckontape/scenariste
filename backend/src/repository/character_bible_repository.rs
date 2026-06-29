use sqlx::PgPool;
use uuid::Uuid;
use crate::models::character_bible::{CharacterBible, CreateCharacterInput, UpdateCharacterInput};

pub struct CharacterBibleRepository;

impl CharacterBibleRepository {
    pub async fn list_by_project(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<CharacterBible>, sqlx::Error> {
        let characters = sqlx::query_as!(
            CharacterBible,
            r#"
            SELECT 
                cb.id, cb.project_id, cb.full_name, cb.character_role as "character_role!", cb.avatar_url, cb.description, cb.attributes, cb.created_at, cb.updated_at
            FROM character_bible cb
            JOIN project_members pm ON cb.project_id = pm.project_id
            WHERE cb.project_id = $1 AND pm.user_id = $2
            ORDER BY cb.full_name ASC
            "#,
            project_id,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(characters)
    }

    pub async fn find_by_id(pool: &PgPool, character_id: Uuid, user_id: Uuid) -> Result<Option<CharacterBible>, sqlx::Error> {
        let character = sqlx::query_as!(
            CharacterBible,
            r#"
            SELECT 
                cb.id, cb.project_id, cb.full_name, cb.character_role as "character_role!", cb.avatar_url, cb.description, cb.attributes, cb.created_at, cb.updated_at
            FROM character_bible cb
            JOIN project_members pm ON cb.project_id = pm.project_id
            WHERE cb.id = $1 AND pm.user_id = $2
            "#,
            character_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(character)
    }

    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateCharacterInput) -> Result<CharacterBible, sqlx::Error> {
        // Vérification de l'accès au projet (owner ou write)
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
            return Err(sqlx::Error::RowNotFound);
        }

        let character = sqlx::query_as!(
            CharacterBible,
            r#"
            INSERT INTO character_bible (project_id, full_name, character_role, avatar_url, description, attributes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, project_id, full_name, character_role as "character_role!", avatar_url, description, attributes, created_at, updated_at
            "#,
            input.project_id,
            input.full_name,
            input.character_role.unwrap_or_else(|| "protagonist".to_string()),
            input.avatar_url,
            input.description,
            input.attributes.unwrap_or_else(|| serde_json::json!({})),
        )
        .fetch_one(pool)
        .await?;

        Ok(character)
    }

    pub async fn update(pool: &PgPool, character_id: Uuid, user_id: Uuid, input: UpdateCharacterInput) -> Result<CharacterBible, sqlx::Error> {
        let character = sqlx::query_as!(
            CharacterBible,
            r#"
            UPDATE character_bible
            SET 
                full_name = COALESCE($1, full_name),
                character_role = COALESCE($2, character_role),
                avatar_url = COALESCE($3, avatar_url),
                description = COALESCE($4, description),
                attributes = COALESCE($5, attributes),
                updated_at = NOW()
            WHERE id = $6 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $7 AND role IN ('owner', 'write')
            )
            RETURNING id, project_id, full_name, character_role as "character_role!", avatar_url, description, attributes, created_at, updated_at
            "#,
            input.full_name,
            input.character_role,
            input.avatar_url,
            input.description,
            input.attributes,
            character_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(character)
    }

    pub async fn delete(pool: &PgPool, character_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM character_bible 
            WHERE id = $1 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $2 AND role IN ('owner', 'write')
            )
            "#,
            character_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
