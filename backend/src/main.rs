use axum::{routing::{get, post}, Router}; // 🔴 CORRECTION 1 : Ajout de ", post" ici
use std::net::SocketAddr;
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method, HeaderValue};

mod db;
mod models;
mod handler;
mod service;
mod repository;
// 🔴 CORRECTION 2 : "handler" au singulier pour correspondre à votre dossier !

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL doit être définie dans le fichier .env");

    println!("🔄 Initialisation de la base de données...");
    let pool = db::init_pool(&database_url).await?;
    println!("✅ Base de données prête et migrations appliquées.");

    // Configuration CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    // Configuration des routes Axum
    let app = Router::new()
        .route("/api/status", get(|| async { "{\"status\": \"OK\"}" }))
        .route("/api/auth/register", post(handler::auth::register))
        .route("/api/auth/login", post(handler::auth::login))
        .route("/api/auth/protected", get(handler::auth::protected_route))
        .route("/api/auth/me", get(handler::auth::get_current_user))
        .route("/api/auth/profile", post(handler::auth::upsert_profile))
        .route("/api/articles", get(handler::article::list_articles).post(handler::article::create_article))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🖥️  Serveur Rustover démarré sur l'adresse : {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}