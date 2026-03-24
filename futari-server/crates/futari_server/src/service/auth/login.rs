use crate::repository::user::repository_find_user_by_email;
use crate::service::auth::session::SessionService;
use crate::service::auth::totp::TotpTempToken;
use tracing::info;
use futari_dto::auth::request::LoginRequest;
use futari_errors::errors::{Errors, ServiceResult};

use crate::utils::crypto::password::verify_password;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

/// 로그인 결과: 세션 생성 또는 TOTP 필요
pub enum LoginResult {
    /// TOTP 없음: 세션 ID 반환
    SessionCreated {
        session_id: String,
        remember_me: bool,
    },
    /// TOTP 필요: 임시 토큰 반환
    TotpRequired(String),
}

/// 로그인 요청을 처리한다.
///
/// # 역할
/// - 이메일/비밀번호 자격 증명을 검증한다.
/// - TOTP 활성 사용자면 임시 토큰 발급 후 TOTP 단계를 요구한다.
/// - TOTP 비활성 사용자면 즉시 세션을 생성한다.
///
/// # 연계
/// - `repository_find_user_by_email`
/// - `verify_password`
/// - `TotpTempToken::create`
/// - `SessionService::create_session`
///
/// # Errors
/// - 인증 실패 시 `Errors::InvalidCredentials`
/// - 세션/토큰 저장 실패 시 Redis/저장소 에러를 반환한다.
pub async fn service_login(
    db: &DatabaseConnection,
    redis: &ConnectionManager,
    payload: LoginRequest,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> ServiceResult<LoginResult> {
    // 사용자 검증 (email enumeration 방지: 모든 실패를 동일한 에러로 반환)
    let user = repository_find_user_by_email(db, payload.email.clone())
        .await?
        .ok_or(Errors::InvalidCredentials)?;

    // 비밀번호 검증 (실패 시 동일한 에러 반환)
    let password_hash = user.password.ok_or(Errors::InvalidCredentials)?;
    verify_password(&payload.password, &password_hash).map_err(|_| Errors::InvalidCredentials)?;

    // TOTP 활성화 확인
    if user.totp_enabled_at.is_some() {
        // TOTP 필요: 임시 토큰 생성
        let temp_token =
            TotpTempToken::create(redis, user.id, user_agent, ip_address, payload.remember_me)
                .await?;

        info!(user_id = %user.id, "Login requires TOTP");
        return Ok(LoginResult::TotpRequired(temp_token.token));
    }

    // TOTP 없음: 바로 세션 생성
    let session =
        SessionService::create_session(redis, user.id.to_string(), user_agent, ip_address).await?;

    info!(user_id = %user.id, "Login successful");

    Ok(LoginResult::SessionCreated {
        session_id: session.session_id,
        remember_me: payload.remember_me,
    })
}
