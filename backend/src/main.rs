use axum::{routing::{get, post}, Json, Router}; // <-- On importe `post`
use serde::{Deserialize, Serialize}; // <-- On ajoute `Deserialize` pour lire le JSON
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    version: String,
    online: bool,
}

// 1. La structure des données que l'on s'attend à RECEVOIR d'Angular
// #[derive(Deserialize)] permet à Serde de transformer le JSON reçu en structure Rust
#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    entreprise: String,
}

// 2. La structure de la réponse que l'on va RENVOYER après inscription
#[derive(Serialize)]
struct RegisterResponse {
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // On ajoute notre nouvelle route POST : /api/register
    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/register", post(register_user)) // <-- Nouvelle route
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Serveur Rust démarré sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: String::from("Le back-end Rust fonctionne parfaitement !"),
        version: String::from("0.1.0"),
        online: true,
    })
}

// 3. Le Handler pour l'inscription
// Axum utilise "Json(payload)" pour intercepter et parser automatiquement le corps de la requête
async fn register_user(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    println!("📩 Inscription reçue pour l'email : {}", payload.email);
    println!("🏢 Entreprise : {}", payload.entreprise);

    // Ici, on simulera plus tard l'insertion en BDD.
    // Pour l'instant, on répond avec un succès.
    let response = RegisterResponse {
        success: true,
        message: format!("Le compte pour {} a bien été créé !", payload.email),
    };

    Json(response)
}