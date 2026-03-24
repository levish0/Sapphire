use super::super::filter::{ActionLogFilter, apply_action_log_filter};
use crate::repository::common::repository_query_exists;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::action_logs::{Column as ActionLogColumn, Entity as ActionLogEntity};
use futari_errors::errors::Errors;

/// 현재 커서보다 과거 액션 로그 존재 여부를 확인한다.
///
/// # 역할
/// `id < cursor_id` + 동일 필터 조건으로 존재 여부만 조회한다.
///
/// # 연계
/// - `service_get_action_logs`
/// - `repository_query_exists`
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_exists_older_action_log<C>(
    conn: &C,
    filter: &ActionLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_action_log_filter(
        ActionLogEntity::find().filter(ActionLogColumn::Id.lt(cursor_id)),
        filter,
    );

    repository_query_exists(conn, query).await
}
