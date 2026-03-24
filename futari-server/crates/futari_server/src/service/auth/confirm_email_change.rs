use crate::repository::user::{
    UserUpdateParams, repository_find_user_by_email, repository_update_user,
};
use crate::service::auth::change_email::EmailChangeData;
use crate::utils::redis_cache::get_json_and_delete;
use redis::aio::ConnectionManager;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;
use futari_errors::errors::{Errors, ServiceResult};

/// 이메일 변경을 확인합니다.
pub async fn service_confirm_email_change(
    db: &DatabaseConnection,
    redis_conn: &ConnectionManager,
    token: &str,
) -> ServiceResult<()> {
    // 1. Redis에서 토큰 검증 (get_del로 일회용)
    let token_key = futari_constants::email_change_key(token);
    let change_data: EmailChangeData = get_json_and_delete(
        redis_conn,
        &token_key,
        || Errors::TokenInvalidEmailChange,
        |_| Errors::TokenInvalidEmailChange,
    )
    .await?;

    // 2. user_id 파싱
    let user_id =
        Uuid::parse_str(&change_data.user_id).map_err(|_| Errors::TokenInvalidEmailChange)?;

    let txn = db.begin().await?;

    // 3. 이메일 중복 체크 (토큰 발급 후 다른 사용자가 해당 이메일을 사용했을 수 있음)
    if let Some(existing) =
        repository_find_user_by_email(&txn, change_data.new_email.clone()).await?
        && existing.id != user_id
    {
        return Err(Errors::UserEmailAlreadyExists);
    }

    // 4. 이메일 업데이트
    repository_update_user(
        &txn,
        user_id,
        UserUpdateParams {
            email: Some(change_data.new_email.clone()),
            ..Default::default()
        },
    )
    .await?;

    txn.commit().await?;

    info!(user_id = %user_id, "Email changed");

    Ok(())
}
