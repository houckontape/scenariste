// backend/src/handler/article.rs
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::article::{Article, CreateArticleInput};
use crate::models::auth::Claims;
use crate::models::user::UserRole;

/// GET /api/articles
/// Liste tous les articles (notes de brainstorming) par ordre chronologique décroissant.
pub async fn list_articles(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Article>>, (StatusCode, String)> {
    let articles = sqlx::query_as!(
        Article,
        r#"
        SELECT id, project_id, author_id, title, content, tags, created_at, updated_at
        FROM brainstorming_notes
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur BDD : {}", e)))?;

    Ok(Json(articles))
}

/// POST /api/articles
/// Crée un nouvel article. Réservé aux super_admin.
pub async fn create_article(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateArticleInput>,
) -> Result<(StatusCode, Json<Article>), (StatusCode, String)> {
    // 1. RBAC : Vérification du rôle avant toute opération
    if claims.role != UserRole::SuperAdmin {
        return Err((
            StatusCode::FORBIDDEN,
            "Accès refusé : Seuls les super_admin peuvent créer des articles.".to_string(),
        ));
    }

    // 2. Insertion en BDD via compile-time query
    let article = sqlx::query_as!(
        Article,
        r#"
        INSERT INTO brainstorming_notes (project_id, author_id, title, content, tags)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, project_id, author_id, title, content, tags, created_at, updated_at
        "#,
        payload.project_id,
        claims.sub, // L'UUID de l'auteur extrait du JWT
        payload.title,
        payload.content,
        &payload.tags[..] // Conversion en slice pour SQLx
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur insertion BDD : {}", e)))?;

    Ok((StatusCode::CREATED, Json(article)))
}
