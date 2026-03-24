use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::common::Role;
use futari_entity::user_roles::{Column, Entity};
use futari_errors::errors::Errors;

/// 특정 역할에 할당된 활성 사용자 ID 목록 조회
pub async fn repository_find_active_user_ids_by_role_name<C>(
    conn: &C,
    role: Role,
) -> Result<Vec<Uuid>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let roles = Entity::find()
        .filter(Column::Role.eq(role))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .all(conn)
        .await?;

    Ok(roles.into_iter().map(|r| r.user_id).collect())
}
