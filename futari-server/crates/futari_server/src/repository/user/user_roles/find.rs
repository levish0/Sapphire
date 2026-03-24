use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::common::Role;
use futari_entity::user_roles::{Column, Entity};
use futari_errors::errors::Errors;

/// 사용자의 유효한 역할 목록 조회 (만료되지 않은 것만)
pub async fn repository_find_user_roles<C>(conn: &C, user_id: Uuid) -> Result<Vec<Role>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let mut roles = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .all(conn)
        .await?
        .into_iter()
        .map(|e| e.role)
        .collect::<Vec<_>>();

    roles.sort_by_key(|role| std::cmp::Reverse(role.display_priority()));

    Ok(roles)
}
