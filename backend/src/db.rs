use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

// Nous définissons un "Type Alias" pour rendre notre code plus lisible
pub type DbPool = Pool<Postgres>;

/// Initialise la connexion à PostgreSQL et applique les migrations
pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    // Exécution des migrations internes
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            eprintln!("❌ Erreur lors des migrations : {}", e);
            sqlx::Error::WorkerCrashed // On propage une erreur SQLx compatible
        })?;

    Ok(pool)
}