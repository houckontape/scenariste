use axum::{routing::get, Router};
use std::net::SocketAddr;
use dotenvy::dotenv;

// Déclaration de notre module de base de données
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Chargement du fichier .env
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL doit être définie dans le fichier .env");

    println!("🔄 Initialisation de la base de données...");
    let pool = db::init_pool(&database_url).await?;
    println!("✅ Base de données prête et migrations appliquées.");

    // 2. Configuration des routes Axum
    let app = Router::new()
        .route("/api/status", get(|| async { "{\"status\": \"OK\"}" }))
        .with_state(pool);

    // 3. Démarrage du serveur façon Axum v0.7 (via Tokio TcpListener)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Pour éviter le warning de RustRover sur le HTTP non sécurisé dans les logs,
    // on écrit simplement l'adresse sans le protocole http:// ou on l'affiche explicitement.
    println!("🖥️  Serveur Rustover démarré sur l'adresse : {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}