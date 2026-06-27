use axum::{routing::{get, post}, Router}; // 🔴 CORRECTION 1 : Ajout de ", post" ici
use std::net::SocketAddr;
use dotenvy::dotenv;

mod db;
mod models;
mod handler; // 🔴 CORRECTION 2 : "handler" au singulier pour correspondre à votre dossier !

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL doit être définie dans le fichier .env");

    println!("🔄 Initialisation de la base de données...");
    let pool = db::init_pool(&database_url).await?;
    println!("✅ Base de données prête et migrations appliquées.");

    // Configuration des routes Axum
    let app = Router::new()
        .route("/api/status", get(|| async { "{\"status\": \"OK\"}" }))
        // 🔴 CORRECTION 3 : Utilisation de handler::auth au singulier
        .route("/api/auth/register", post(handler::auth::register))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🖥️  Serveur Rustover démarré sur l'adresse : {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}