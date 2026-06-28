use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::article::Article;

pub async fn list_articles(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Article>>, StatusCode> {

    // Requête compile-time sécurisée pour récupérer les articles triés par date
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, content, author_name, published_at FROM articles ORDER BY published_at DESC"
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(articles))
}