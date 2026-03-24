use super::super::filter::{ModerationLogFilter, apply_moderation_log_filter};
use crate::repository::common::repository_query_exists;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::moderation_logs::{Column as ModerationLogColumn, Entity as ModerationLogEntity};
use futari_errors::errors::Errors;

/// 현재 커서보다 최신 모더레이션 로그 존재 여부를 확인한다.
///
/// # 역할
/// `id > cursor_id` + 동일 필터 조건으로 존재 여부만 조회한다.
///
/// # 연계
/// - `service_list_moderation_logs`
/// - `repository_query_exists`
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_exists_newer_moderation_log<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_moderation_log_filter(
        ModerationLogEntity::find().filter(ModerationLogColumn::Id.gt(cursor_id)),
        filter,
    );

    repository_query_exists(conn, query).await
}
