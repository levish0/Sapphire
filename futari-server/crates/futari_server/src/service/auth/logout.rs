use crate::service::auth::session::SessionService;
use redis::aio::ConnectionManager;
use tracing::info;
use futari_errors::errors::ServiceResult;

/// 현재 세션을 로그아웃 처리한다.
///
/// # 역할
/// 세션 ID에 해당하는 서버 세션을 삭제한다.
///
/// # 연계
/// - `SessionService::delete_session`
///
/// # Errors
/// - 세션 삭제 실패 시 Redis/저장소 에러를 반환한다.
pub async fn service_logout(redis: &ConnectionManager, session_id: &str) -> ServiceResult<()> {
    // 세션 삭제 (delete_session 내부에서 유효성 확인)
    SessionService::delete_session(redis, session_id).await?;

    info!(session_id = %session_id, "Logout");

    Ok(())
}
