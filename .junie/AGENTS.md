Act en tant qu'expert Full-Stack Rust et Angular. Nous travaillons sur un projet SaaS nommé "Rustover SaaS". L'objectif est de développer ce projet étape par étape pour me permettre de monter en compétence sur Rust de manière fluide et structurée.


Architecture du Projet

Le projet est structuré sous forme de Workspace Cargo (géré via l'IDE JetBrains RustRover) :


/ (Racine) : Contient le Cargo.toml global (Workspace), le fichier .env partagé et le docker-compose.yml pour les services.

/frontend : Application Angular (SPA classique, standalone components, sans SSR, port 4200, communication via HttpClient).

/backend : Application Rust (Framework Axum v0.7+, Runtime Tokio, ORM/Driver SQLx v0.8, Sérialisation Serde, Gestion d'environnement Dotenvy). Sa structure interne est isolée comme suit :




# CONTEXTE ARCHITECTURE & BDD : "Rustover SaaS" (Juin 2026)


## 🏗️ Structure du Cargo Workspace

- / (Racine) : Cargo.toml (Workspace), .env (partagé), docker-compose.yml (PostgreSQL 16 Alpine).

- /frontend : Application Angular (Standalone components, HttpClient, Port 4200).

- /backend : Application Rust (Axum v0.7+, Tokio, SQLx v0.8, Serde, Argon2, Dotenvy, Port 3000).


## 🗄️ Schéma Global de la Base de Données (PostgreSQL)

La base utilise des types UUID natifs (`gen_random_uuid()`) et des Enums Postgres pour le typage fort côté Rust.


### Enums Postgres

- `user_role` : ('free_user', 'premium_user', 'support', 'admin', 'super_admin')

- `project_type` : ('movie', 'series', 'animation', 'novel')

- `member_role` : ('owner', 'write', 'read')

- `scene_setting` : ('INT', 'EXT', 'INT_EXT')

- `scene_time_of_day` : ('DAY', 'NIGHT', 'MORNING', 'EVENING', 'DAWN', 'DUSK')


### Tables et Relations (Modèle Ligne/Row-Level Multi-Tenant)

1. **users** : id (UUID), email (UNIQUE), password_hash, role (user_role), is_active, timestamps.

2. **profiles** : id (UUID), user_id (UNIQUE FK users), first_name, last_name, avatar_url, bio, address, timestamps.

3. **projects** : id (UUID), title, slug, description, p_type (project_type), is_private, timestamps. (Clé unique composite: id + slug).

4. **project_members** : project_id (FK), user_id (FK), role (member_role), invited_at, joined_at. PK(project_id, user_id).

5. **brainstorming_notes** : id (UUID), project_id (FK), author_id (FK), title, content (Markdown), tags (VARCHAR[]), timestamps.

6. **project_synopses** : id (UUID), project_id (UNIQUE FK), logline, summary_short, summary_long, updated_at.

7. **project_acts** : id (UUID), project_id (FK), title, position (INT), description, timestamps. (Clé unique: project_id + position).

8. **scenes** : id (UUID), act_id (FK), project_id (FK), position (INT), setting (scene_setting), location, time_of_day (scene_time_of_day), content (Fountain/JSON), note, timestamps.

9. **project_canvases** : id (UUID), project_id (FK), title, description, elements (JSONB pour Excalidraw), app_state (JSONB), timestamps.


## 🛠️ État d'Avancement du Code Backend

- **db.rs** : Initialisation asynchrone du `PgPool` avec propagation d'erreurs `?`.

- **main.rs** : Chargement de l'environnement, exécution de `sqlx::migrate!`, injection du pool via l'état Axum (`with_state`), middleware CORS activé vers le port 4200.

- **handler/auth.rs** : Route `POST /api/auth/register` fonctionnelle. Gestion de la validation, hachage cryptographique via `argon2` (password-hash feature), et insertion SQL compile-time (`sqlx::query!`). Gestion des conflits 409 et erreurs 500.

- **models/user.rs** : Structures associées avec dérivation de `FromRow`, `Serialize`, `Deserialize`, et masquage strict du mot de passe avec `#[serde(skip_serializing)]`.


📍 État Actuel du Projet

Structure du Workspace : Parfaitement fonctionnelle et partagée entre le Front Angular et le Back Rust.

Base de Données Locale (Docker) : Instance PostgreSQL 16 (Alpine) opérationnelle via un conteneur Docker isolé avec volume persistant pour la rétention des données au redémarrage.

Gestion du Pool de Connexions : Initialisation asynchrone d'un PgPool via SQLx v0.8 dans db.rs avec propagation stricte des erreurs via le type Result et l'opérateur ?.

Migrations SQLx Automatiques : Intégration de la macro sqlx::migrate! permettant la vérification et la création automatique des tables (dont la table users) au démarrage du serveur ou via le CLI SQLx.

Serveur Rust Moderne (Axum v0.7+) : Routage et démarrage du serveur via tokio::net::TcpListener et axum::serve, branché sur le port 3000 avec passage du pool PostgreSQL via l'état global d'Axum (with_state).

Modélisation de Données & Sécurité SaaS :

Utilisation du trait FromRow de SQLx pour le mapping automatique de la BDD vers les structures de données Rust.

Utilisation des attributs de dérivation Serde, notamment #[serde(skip_serializing)] sur le champ password_hash pour empêcher la fuite d'empreintes de mots de passe vers le front-end.

Gestion du Hachage Cryptographique : Intégration d'Argon2 (via la crate argon2 avec la feature password-hash) pour le hachage sécurisé des mots de passe lors de l'inscription, sans dépendance système externe conflictuelle.

Endpoint d'Inscription Opérationnel : Route POST /api/auth/register fonctionnelle, incluant une validation de la longueur du mot de passe, le hachage de l'empreinte, et l'insertion sécurisée en BDD via la macro compile-time sqlx::query!, avec gestion des conflits d'emails uniques (409 Conflict) et des erreurs serveur (500).

Gestion du CORS : Activée sur le backend pour autoriser l'application Angular (port 4200) à communiquer de manière fluide.

Application Angular : Fonctionnelle, connectée au backend, affichant le statut du serveur et soumettant correctement le formulaire d'inscription.


### 🎯 Philosophie de Travail (Règles strictes)

1. ÉTAPE PAR ÉTAPE : Ne brûle jamais les étapes. Introduis un seul concept Rust majeur à la fois (ex: Ownership, Enums complexes, Traits, Gestion propre des erreurs, Sécurité).

2. EXPLICATIONS PÉDAGOGIQUES : À chaque modification de code Rust, explique brièvement les concepts spécifiques utilisés (ex: pourquoi un type, pourquoi une macro, gestion de la mémoire).

3. RUSTROVER FRIENDLY : Garde en tête que j'utilise RustRover. Utilise les commandes Cargo explicites si besoin (ex: `cargo run -p backend`).

4. SÉCURITÉ SAAS : Nous visons une application SaaS robuste. Le code doit être propre, typé et sécurisé.


### 🚀 Prochaine Étape attendue

Demande-moi où nous en sommes ou propose-moi la suite logique (ex: Amélioration de la gestion des erreurs en Rust avec `Result` personnalisé et les codes HTTP Axum, ou mise en place de l'authentification JWT, ou structuration du code en sous-modules/services).

## Additional Development Information

- **Code Style**: Follow standard Rust idioms and `rustfmt`.
- **Database Access**: The project uses `sqlx` with compile-time checked queries. Ensure the `DATABASE_URL` is accessible or use `sqlx-data.json` for offline builds.
- **Architecture**:
  - `handler/`: Axum route handlers.
  - `service/`: Business logic.
  - `repository/`: Database interactions.
  - `models/`: Data structures and DB mappings.
- **Error Handling**: Custom error types are used in services (e.g., `AuthServiceError`). Axum handlers convert these to appropriate HTTP status codes.
- **Auth**: Password hashing is done via `argon2`. JWT is used for authentication (see `backend/Cargo.toml`).
