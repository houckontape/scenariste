use sqlx::PgPool;
use uuid::Uuid;
use crate::models::brainstorming::{BrainstormingNote, CreateBrainstormingNoteInput, UpdateBrainstormingNoteInput};

pub struct BrainstormingRepository;

impl BrainstormingRepository {
    pub async fn list_by_project(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<Vec<BrainstormingNote>, sqlx::Error> {
        let notes = sqlx::query_as!(
            BrainstormingNote,
            r#"
            SELECT 
                bn.id, bn.project_id, bn.author_id, bn.title, bn.content, bn.tags, bn.created_at, bn.updated_at
            FROM brainstorming_notes bn
            JOIN project_members pm ON bn.project_id = pm.project_id
            WHERE bn.project_id = $1 AND pm.user_id = $2
            ORDER BY bn.created_at DESC
            "#,
            project_id,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(notes)
    }

    pub async fn find_by_id(pool: &PgPool, note_id: Uuid, user_id: Uuid) -> Result<Option<BrainstormingNote>, sqlx::Error> {
        let note = sqlx::query_as!(
            BrainstormingNote,
            r#"
            SELECT 
                bn.id, bn.project_id, bn.author_id, bn.title, bn.content, bn.tags, bn.created_at, bn.updated_at
            FROM brainstorming_notes bn
            JOIN project_members pm ON bn.project_id = pm.project_id
            WHERE bn.id = $1 AND pm.user_id = $2
            "#,
            note_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(note)
    }

    pub async fn create(pool: &PgPool, author_id: Uuid, input: CreateBrainstormingNoteInput) -> Result<BrainstormingNote, sqlx::Error> {
        // Vérification de l'accès au projet (owner ou write)
        let has_access = sqlx::query!(
            r#"
            SELECT 1 as "access!" FROM project_members 
            WHERE project_id = $1 AND user_id = $2 AND role IN ('owner', 'write')
            "#,
            input.project_id,
            author_id
        )
        .fetch_optional(pool)
        .await?;

        if has_access.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let note = sqlx::query_as!(
            BrainstormingNote,
            r#"
            INSERT INTO brainstorming_notes (project_id, author_id, title, content, tags)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, project_id, author_id, title, content, tags, created_at, updated_at
            "#,
            input.project_id,
            author_id,
            input.title,
            input.content,
            input.tags.as_deref()
        )
        .fetch_one(pool)
        .await?;

        Ok(note)
    }

    pub async fn update(pool: &PgPool, note_id: Uuid, user_id: Uuid, input: UpdateBrainstormingNoteInput) -> Result<BrainstormingNote, sqlx::Error> {
        let note = sqlx::query_as!(
            BrainstormingNote,
            r#"
            UPDATE brainstorming_notes
            SET 
                title = COALESCE($1, title),
                content = COALESCE($2, content),
                tags = COALESCE($3, tags),
                updated_at = NOW()
            WHERE id = $4 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $5 AND role IN ('owner', 'write')
            )
            RETURNING id, project_id, author_id, title, content, tags, created_at, updated_at
            "#,
            input.title,
            input.content,
            input.tags.as_deref(),
            note_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(note)
    }

    pub async fn delete(pool: &PgPool, note_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM brainstorming_notes 
            WHERE id = $1 AND project_id IN (
                SELECT project_id FROM project_members 
                WHERE user_id = $2 AND role IN ('owner', 'write')
            )
            "#,
            note_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
