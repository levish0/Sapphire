use crate::repository::user::UserUpdateParams;
use crate::repository::user::repository_get_user_by_id;
use crate::repository::user::repository_update_user;
use crate::service::auth::session::SessionService;
use crate::utils::crypto::password::{hash_password, verify_password};
use redis::aio::ConnectionManager;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;
use futari_dto::auth::request::ChangePasswordRequest;
use futari_errors::errors::{Errors, ServiceResult};

/// 비밀번호를 변경합니다.
///
/// # Arguments
/// * `db` - 데이터베이스 연결
/// * `redis_conn` - Redis 연결
/// * `user_id` - 사용자 ID
/// * `session_id` - 현재 세션 ID (유지할 세션)
/// * `payload` - 비밀번호 변경 요청
pub async fn service_change_password(
    db: &DatabaseConnection,
    redis_conn: &ConnectionManager,
    user_id: Uuid,
    session_id: &str,
    payload: ChangePasswordRequest,
) -> ServiceResult<()> {
    let txn = db.begin().await?;

    // 1. 사용자 조회
    let user = repository_get_user_by_id(&txn, user_id).await?;

    // 2. 비밀번호가 설정되어 있는지 확인 (OAuth 전용 사용자 제외)
    let password_hash = user.password.ok_or(Errors::UserPasswordNotSet)?;

    // 3. 현재 비밀번호 검증
    verify_password(&payload.current_password, &password_hash)?;

    // 4. 새 비밀번호가 현재 비밀번호와 동일한지 확인
    if payload.current_password == payload.new_password {
        return Err(Errors::BadRequestError(
            "New password must be different from current password.".to_string(),
        ));
    }

    // 5. 새 비밀번호 해싱
    let new_password_hash = hash_password(&payload.new_password)?;

    // 6. 비밀번호 업데이트
    repository_update_user(
        &txn,
        user_id,
        UserUpdateParams {
            password: Some(Some(new_password_hash)),
            ..Default::default()
        },
    )
    .await?;

    txn.commit().await?;

    // 7. 현재 세션을 제외한 모든 세션 무효화
    let deleted_count =
        SessionService::delete_other_sessions(redis_conn, &user_id.to_string(), session_id).await?;

    info!(user_id = %user_id, invalidated_sessions = deleted_count, "Password changed");

    Ok(())
}
