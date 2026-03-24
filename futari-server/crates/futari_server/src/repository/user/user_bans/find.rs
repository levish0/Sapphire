use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::user_bans::{Column, Entity, Model};
use futari_errors::errors::Errors;

/// 사용자의 유효한 차단 정보 조회 (만료되지 않은 것)
pub async fn repository_find_user_ban<C>(conn: &C, user_id: Uuid) -> Result<Option<Model>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let ban = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .one(conn)
        .await?;

    Ok(ban)
}

/// 사용자가 차단 상태인지 확인
pub async fn repository_is_user_banned<C>(conn: &C, user_id: Uuid) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let ban = repository_find_user_ban(conn, user_id).await?;
    Ok(ban.is_some())
}
