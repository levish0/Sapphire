use super::filter::{ModerationLogFilter, apply_moderation_log_filter};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;
use futari_dto::pagination::CursorDirection;
use futari_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity, Model as ModerationLogModel,
};
use futari_errors::errors::Errors;

/// 모더레이션 로그 목록을 필터/커서 조건으로 조회한다.
///
/// # 역할
/// - `ModerationLogFilter`를 적용한다.
/// - 커서 방향에 맞는 정렬/경계 조건을 적용한다.
/// - 커서가 없으면 최신순(`id DESC`)으로 조회한다.
///
/// # 연계
/// - `service_list_moderation_logs`
/// - `apply_moderation_log_filter`
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_find_moderation_logs<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<ModerationLogModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = apply_moderation_log_filter(ModerationLogEntity::find(), filter);

    // Apply cursor-based filtering (UUIDv7 is time-sortable)
    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(ModerationLogColumn::Id.lt(id))
                .order_by_desc(ModerationLogColumn::Id),
            CursorDirection::Newer => query
                .filter(ModerationLogColumn::Id.gt(id))
                .order_by_asc(ModerationLogColumn::Id),
        };
    } else {
        query = query.order_by_desc(ModerationLogColumn::Id);
    }

    let results = query.limit(limit).all(conn).await?;

    Ok(results)
}
