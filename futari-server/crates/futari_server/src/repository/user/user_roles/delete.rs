use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::common::Role;
use futari_entity::user_roles::{Column, Entity};
use futari_errors::errors::Errors;

/// 사용자의 특정 역할 삭제 (만료 포함 전체)
pub async fn repository_delete_user_role<C>(
    conn: &C,
    user_id: Uuid,
    role: Role,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let result = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Role.eq(role))
        .exec(conn)
        .await?;

    Ok(result.rows_affected)
}

/// 사용자의 특정 역할 중 만료된 것만 삭제 (grant 시 UNIQUE 충돌 방지)
pub async fn repository_delete_expired_user_role<C>(
    conn: &C,
    user_id: Uuid,
    role: Role,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let result = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Role.eq(role))
        .filter(Column::ExpiresAt.is_not_null())
        .filter(Column::ExpiresAt.lte(now))
        .exec(conn)
        .await?;

    Ok(result.rows_affected)
}
