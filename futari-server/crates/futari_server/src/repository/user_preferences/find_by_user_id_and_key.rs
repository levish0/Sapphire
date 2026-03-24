use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_constants::UserPreferenceKey;
use futari_entity::user_preferences::{
    Column as UserPreferenceColumn, Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use futari_errors::errors::Errors;

/// 사용자 환경설정 단건을 사용자 ID와 키로 조회한다.
///
/// # 역할
/// `user_id + key` 조합의 설정 한 건을 조회한다.
///
/// # 연계
/// - `service_get_user_preference`
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_find_user_preference_by_user_id_and_key<C>(
    conn: &C,
    user_id: Uuid,
    key: UserPreferenceKey,
) -> Result<Option<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    let preference = UserPreferenceEntity::find()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .filter(UserPreferenceColumn::Key.eq(key.as_str()))
        .one(conn)
        .await?;

    Ok(preference)
}
