use crate::extractors::RequiredSession;
use crate::service::user::management::ban_user::service_ban_user;
use crate::state::AppState;
use crate::utils::extract::extract_ip_address::extract_ip_address;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use std::net::SocketAddr;
use futari_dto::user::request::BanUserRequest;
use futari_dto::user::response::BanUserResponse;
use futari_dto::validator::json_validator::ValidatedJson;
use futari_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/users/ban",
    request_body = BanUserRequest,
    responses(
        (status = 200, description = "User banned successfully", body = BanUserResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Login required", body = ErrorResponse),
        (status = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "Not Found - User not found", body = ErrorResponse),
        (status = 409, description = "Conflict - User already banned", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Management"
)]
pub async fn ban_user(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
    ValidatedJson(payload): ValidatedJson<BanUserRequest>,
) -> Result<BanUserResponse, Errors> {
    let ip_address = extract_ip_address(&headers, addr);

    service_ban_user(
        &state.db,
        payload.user_id,
        payload.expires_at,
        payload.reason,
        &session,
        &ip_address,
    )
    .await
}
