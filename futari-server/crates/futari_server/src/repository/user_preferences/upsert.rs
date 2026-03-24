use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, EntityTrait, JsonValue, Set};
use uuid::Uuid;
use futari_constants::UserPreferenceKey;
use futari_entity::user_preferences::{
    ActiveModel as UserPreferenceActiveModel, Column as UserPreferenceColumn,
    Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use futari_errors::errors::Errors;

/// 사용자 환경설정 단건을 upsert 한다.
///
/// # 역할
/// `user_id + key` 충돌 시 값을 갱신하고, 없으면 새 레코드를 생성한다.
///
/// # 연계
/// - `service_set_user_preference`
///
/// # Errors
/// - 저장 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_upsert_user_preference<C>(
    conn: &C,
    user_id: Uuid,
    key: UserPreferenceKey,
    value: JsonValue,
) -> Result<UserPreferenceModel, Errors>
where
    C: ConnectionTrait,
{
    let active_model = UserPreferenceActiveModel {
        user_id: Set(user_id),
        key: Set(key.to_string()),
        value: Set(value),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let result = UserPreferenceEntity::insert(active_model)
        .on_conflict(
            OnConflict::columns([UserPreferenceColumn::UserId, UserPreferenceColumn::Key])
                .update_columns([UserPreferenceColumn::Value, UserPreferenceColumn::UpdatedAt])
                .to_owned(),
        )
        .exec_with_returning(conn)
        .await?;

    Ok(result)
}
