use crate::repository::user::{UserUpdateParams, repository_update_user};
use crate::service::auth::forgot_password::PasswordResetData;
use crate::service::auth::session::SessionService;
use crate::utils::crypto::password::hash_password;
use crate::utils::redis_cache::get_json_and_delete;
use redis::aio::ConnectionManager;
use sea_orm::ConnectionTrait;
use tracing::info;
use uuid::Uuid;
use futari_errors::errors::{Errors, ServiceResult};

/// 비밀번호를 재설정합니다.
///
/// # Arguments
/// * `conn` - 데이터베이스 연결
/// * `redis_conn` - Redis 연결
/// * `token` - 비밀번호 재설정 토큰
/// * `new_password` - 새 비밀번호
pub async fn service_reset_password<C>(
    conn: &C,
    redis_conn: &ConnectionManager,
    token: &str,
    new_password: &str,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    // 1. Redis에서 토큰 검증 (get_del로 일회용)
    let token_key = futari_constants::password_reset_key(token);
    let reset_data: PasswordResetData = get_json_and_delete(
        redis_conn,
        &token_key,
        || Errors::TokenInvalidReset,
        |_| Errors::TokenInvalidReset,
    )
    .await?;

    // 2. user_id 파싱
    let user_id = Uuid::parse_str(&reset_data.user_id).map_err(|_| Errors::TokenInvalidReset)?;

    // 3. 새 비밀번호 해싱
    let password_hash = hash_password(new_password)?;

    // 4. 비밀번호 업데이트
    repository_update_user(
        conn,
        user_id,
        UserUpdateParams {
            password: Some(Some(password_hash)),
            ..Default::default()
        },
    )
    .await?;

    // 5. 해당 사용자의 모든 세션 무효화
    let deleted_count =
        SessionService::delete_all_user_sessions(redis_conn, &user_id.to_string()).await?;

    info!(user_id = %user_id, invalidated_sessions = deleted_count, "Password reset completed");

    Ok(())
}
