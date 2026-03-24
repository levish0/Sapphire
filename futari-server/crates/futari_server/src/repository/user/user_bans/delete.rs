use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::user_bans::{Column, Entity};
use futari_errors::errors::Errors;

/// 사용자 차단 해제 (만료 포함 전체)
pub async fn repository_delete_user_ban<C>(conn: &C, user_id: Uuid) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let result = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;

    Ok(result.rows_affected)
}

/// 사용자의 만료된 밴만 삭제 (재밴 시 UNIQUE 충돌 방지)
pub async fn repository_delete_expired_user_ban<C>(conn: &C, user_id: Uuid) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let result = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ExpiresAt.is_not_null())
        .filter(Column::ExpiresAt.lte(now))
        .exec(conn)
        .await?;

    Ok(result.rows_affected)
}
