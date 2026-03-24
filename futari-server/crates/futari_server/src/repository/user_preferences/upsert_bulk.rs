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

/// 사용자 환경설정 다건을 bulk upsert 한다.
///
/// # 역할
/// 전달된 `(key, value)` 목록을 한 번에 insert/update 하며, 빈 입력은 즉시 빈 결과를 반환한다.
///
/// # 연계
/// - `service_set_user_preferences_bulk`
///
/// # Errors
/// - 저장 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_upsert_user_preferences_bulk<C>(
    conn: &C,
    user_id: Uuid,
    preferences: Vec<(UserPreferenceKey, JsonValue)>,
) -> Result<Vec<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    if preferences.is_empty() {
        return Ok(vec![]);
    }

    let now = Utc::now();
    let active_models: Vec<UserPreferenceActiveModel> = preferences
        .into_iter()
        .map(|(key, value)| UserPreferenceActiveModel {
            user_id: Set(user_id),
            key: Set(key.to_string()),
            value: Set(value),
            updated_at: Set(now),
            ..Default::default()
        })
        .collect();

    let results = UserPreferenceEntity::insert_many(active_models)
        .on_conflict(
            OnConflict::columns([UserPreferenceColumn::UserId, UserPreferenceColumn::Key])
                .update_columns([UserPreferenceColumn::Value, UserPreferenceColumn::UpdatedAt])
                .to_owned(),
        )
        .exec_with_returning(conn)
        .await?;

    Ok(results)
}
