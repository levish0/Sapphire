use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::user_roles::{Column, Entity, Model};
use futari_errors::errors::Errors;

/// 사용자의 유효한 역할 목록 조회 (만료되지 않은 것, expires_at 포함)
pub async fn repository_find_user_role_entries<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<Model>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let entries = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .all(conn)
        .await?;

    Ok(entries)
}
