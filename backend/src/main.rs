use axum::{routing::{get, post, patch}, Router};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
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
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    // Configuration des routes Axum
    let app = Router::new()
        .route("/api/status", get(|| async { "{\"status\": \"OK\"}" }))
        .route("/api/auth/register", post(handler::auth::register))
        .route("/api/auth/login", post(handler::auth::login))
        .route("/api/auth/protected", get(handler::auth::protected_route))
        .route("/api/auth/me", get(handler::auth::get_current_user))
        .route("/api/auth/profile", post(handler::auth::upsert_profile))
        .route("/api/articles", get(handler::article::list_articles).post(handler::article::create_article))
        .route("/api/projects", get(handler::project::list_projects).post(handler::project::create_project))
        .route("/api/projects/:id", get(handler::project::get_project).patch(handler::project::update_project).delete(handler::project::delete_project))
        .route("/api/projects/:id/acts", get(handler::project_act::list_acts))
        .route("/api/acts", post(handler::project_act::create_act))
        .route("/api/acts/:id", get(handler::project_act::get_act).patch(handler::project_act::update_act).delete(handler::project_act::delete_act))
        .route("/api/projects/:id/scenes", get(handler::scene::list_scenes))
        .route("/api/scenes", post(handler::scene::create_scene))
        .route("/api/scenes/:id", get(handler::scene::get_scene).patch(handler::scene::update_scene).delete(handler::scene::delete_scene))
        .route("/api/projects/:id/notes", get(handler::brainstorming::list_notes))
        .route("/api/notes", post(handler::brainstorming::create_note))
        .route("/api/notes/:id", get(handler::brainstorming::get_note).patch(handler::brainstorming::update_note).delete(handler::brainstorming::delete_note))
        .route("/api/projects/:id/members", get(handler::project_member::list_members).post(handler::project_member::add_member))
        .route("/api/projects/:id/members/:user_id", patch(handler::project_member::update_member_role).delete(handler::project_member::remove_member))
        .route("/api/projects/:id/synopsis", get(handler::project_synopsis::get_synopsis).post(handler::project_synopsis::upsert_synopsis).delete(handler::project_synopsis::delete_synopsis))
        .route("/api/metadata/scene-settings", get(handler::metadata::get_scene_settings))
        .route("/api/metadata/scene-times-of-day", get(handler::metadata::get_scene_times_of_day))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🖥️  Serveur Rustover démarré sur l'adresse : {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}