use crate::extractors::RequiredSession;
use crate::service::user::management::unban_user::service_unban_user;
use crate::state::AppState;
use crate::utils::extract::extract_ip_address::extract_ip_address;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use std::net::SocketAddr;
use futari_dto::user::request::UnbanUserRequest;
use futari_dto::user::response::UnbanUserResponse;
use futari_dto::validator::json_validator::ValidatedJson;
use futari_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/users/unban",
    request_body = UnbanUserRequest,
    responses(
        (status = 200, description = "User unbanned successfully", body = UnbanUserResponse),
        (status = 400, description = "Bad request - User is not banned", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Login required", body = ErrorResponse),
        (status = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "Not Found - User not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Management"
)]
pub async fn unban_user(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
    ValidatedJson(payload): ValidatedJson<UnbanUserRequest>,
) -> Result<UnbanUserResponse, Errors> {
    let ip_address = extract_ip_address(&headers, addr);

    service_unban_user(
        &state.db,
        payload.user_id,
        payload.reason,
        &session,
        &ip_address,
    )
    .await
}
