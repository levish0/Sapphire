use crate::bridge::worker_client;
use crate::repository::user::{repository_find_user_by_email, repository_get_user_by_id};
use crate::state::WorkerClient;
use crate::utils::crypto::password::verify_password;
use crate::utils::crypto::token::generate_secure_token;
use crate::utils::redis_cache::issue_token_and_store_json_with_ttl;
use redis::aio::ConnectionManager;
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use futari_config::ServerConfig;
use futari_dto::auth::request::ChangeEmailRequest;
use futari_errors::errors::{Errors, ServiceResult};

#[derive(Debug, Serialize, Deserialize)]
/// Data structure for email change data.
pub struct EmailChangeData {
    pub user_id: String,
    pub new_email: String,
}

/// 이메일 변경을 요청합니다. 새 이메일로 인증 메일이 발송됩니다.
pub async fn service_change_email<C>(
    conn: &C,
    redis_conn: &ConnectionManager,
    worker: &WorkerClient,
    user_id: Uuid,
    payload: ChangeEmailRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    let config = ServerConfig::get();

    // 1. 사용자 조회
    let user = repository_get_user_by_id(conn, user_id).await?;

    // 2. 비밀번호 검증 (OAuth 전용 사용자는 비밀번호 변경 불가)
    let password_hash = user.password.ok_or(Errors::UserPasswordNotSet)?;
    verify_password(&payload.password, &password_hash)?;

    // 3. 새 이메일이 현재 이메일과 동일한지 확인
    if user.email == payload.new_email {
        return Err(Errors::BadRequestError(
            "New email must be different from current email.".to_string(),
        ));
    }

    // 4. 새 이메일이 이미 사용 중인지 확인
    if repository_find_user_by_email(conn, payload.new_email.clone())
        .await?
        .is_some()
    {
        return Err(Errors::UserEmailAlreadyExists);
    }

    // 5. 토큰 생성

    let change_data = EmailChangeData {
        user_id: user.id.to_string(),
        new_email: payload.new_email.clone(),
    };

    // 6. Redis에 토큰 저장 (분 단위 → 초 단위 변환)
    let ttl_seconds = (config.auth_email_change_token_expire_time * 60) as u64;
    let token = issue_token_and_store_json_with_ttl(
        redis_conn,
        generate_secure_token,
        futari_constants::email_change_key,
        &change_data,
        ttl_seconds,
    )
    .await?;

    // 7. Worker 서비스에 이메일 발송 요청 (새 이메일로)
    worker_client::send_email_change_verification(
        worker,
        &payload.new_email,
        &user.handle,
        &token,
        config.auth_email_change_token_expire_time as u64,
    )
    .await?;

    info!(user_id = %user_id, "Email change verification sent");

    Ok(())
}
