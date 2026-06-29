use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::auth::Claims;
use crate::models::project_member::{AddMemberInput, UpdateMemberRoleInput};
use crate::service::project_member_service::{ProjectMemberService, ProjectMemberServiceError};

pub async fn list_members(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectMemberService::list_members(&pool, project_id, claims.sub).await {
        Ok(members) => Ok(Json(members)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn add_member(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<AddMemberInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectMemberService::add_member(&pool, project_id, claims.sub, payload).await {
        Ok(member) => Ok((StatusCode::CREATED, Json(member))),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn update_member_role(
    State(pool): State<PgPool>,
    claims: Claims,
    Path((project_id, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMemberRoleInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectMemberService::update_member_role(&pool, project_id, claims.sub, user_id, payload).await {
        Ok(member) => Ok(Json(member)),
        Err(e) => Err(handle_service_error(e)),
    }
}

pub async fn remove_member(
    State(pool): State<PgPool>,
    claims: Claims,
    Path((project_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ProjectMemberService::remove_member(&pool, project_id, claims.sub, user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(handle_service_error(e)),
    }
}

fn handle_service_error(err: ProjectMemberServiceError) -> (StatusCode, String) {
    match err {
        ProjectMemberServiceError::NotFound => (StatusCode::NOT_FOUND, "Membre non trouvé".to_string()),
        ProjectMemberServiceError::Unauthorized => (StatusCode::FORBIDDEN, "Accès refusé".to_string()),
        ProjectMemberServiceError::UserNotFound => (StatusCode::BAD_REQUEST, "Utilisateur introuvable".to_string()),
        ProjectMemberServiceError::AlreadyMember => (StatusCode::CONFLICT, "L'utilisateur est déjà membre".to_string()),
        ProjectMemberServiceError::LastOwner => (StatusCode::BAD_REQUEST, "Impossible de supprimer le dernier propriétaire".to_string()),
        ProjectMemberServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur serveur : {}", e)),
    }
}
