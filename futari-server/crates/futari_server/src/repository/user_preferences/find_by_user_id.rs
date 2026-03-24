use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::user_preferences::{
    Column as UserPreferenceColumn, Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use futari_errors::errors::Errors;

/// 사용자 환경설정 목록을 사용자 ID로 조회한다.
///
/// # 역할
/// 특정 사용자의 모든 환경설정을 반환한다.
///
/// # 연계
/// - `service_get_user_preferences`
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_find_user_preferences_by_user_id<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    let preferences = UserPreferenceEntity::find()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .all(conn)
        .await?;

    Ok(preferences)
}
