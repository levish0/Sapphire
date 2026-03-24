use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use futari_constants::ModerationAction;
use futari_entity::common::ModerationResourceType;
use futari_entity::moderation_logs::{
    ActiveModel as ModerationLogActiveModel, Model as ModerationLogModel,
};
use futari_errors::errors::Errors;

/// 모더레이션 로그 레코드를 생성한다.
///
/// # 역할
/// 입력 파라미터를 `moderation_logs` 레코드로 변환해 insert 한다.
///
/// # 연계
/// - `service_start_reindex`
/// - 기타 모더레이션 액션 서비스
///
/// # Errors
/// - insert 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_create_moderation_log<C>(
    conn: &C,
    action: ModerationAction,
    actor_id: Option<Uuid>,
    resource_type: ModerationResourceType,
    resource_id: Option<Uuid>,
    reason: String,
    metadata: Option<JsonValue>,
) -> Result<ModerationLogModel, Errors>
where
    C: ConnectionTrait,
{
    let log = ModerationLogActiveModel {
        id: Default::default(),
        action: Set(action.as_str().to_string()),
        actor_id: Set(actor_id),
        resource_type: Set(resource_type),
        resource_id: Set(resource_id),
        reason: Set(reason),
        metadata: Set(metadata),
        created_at: Default::default(),
    };

    let log = log.insert(conn).await?;

    Ok(log)
}
