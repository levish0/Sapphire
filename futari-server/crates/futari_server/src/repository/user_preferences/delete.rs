use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_constants::UserPreferenceKey;
use futari_entity::user_preferences::{Column as UserPreferenceColumn, Entity as UserPreferenceEntity};
use futari_errors::errors::Errors;

/// 사용자 환경설정 단건을 삭제한다.
///
/// # 역할
/// `user_id + key` 조건에 일치하는 레코드를 삭제하고 삭제 여부를 반환한다.
///
/// # 연계
/// - `service_delete_user_preference`
///
/// # Errors
/// - 삭제 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_delete_user_preference<C>(
    conn: &C,
    user_id: Uuid,
    key: UserPreferenceKey,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let result = UserPreferenceEntity::delete_many()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .filter(UserPreferenceColumn::Key.eq(key.as_str()))
        .exec(conn)
        .await?;

    Ok(result.rows_affected > 0)
}
