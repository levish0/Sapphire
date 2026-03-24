use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter};
use futari_entity::common::Role;
use futari_entity::user_roles::{Column, Entity};
use futari_errors::errors::Errors;

/// 특정 역할에 할당된 활성 사용자 수 조회
pub async fn repository_count_active_user_roles_by_role_name<C>(
    conn: &C,
    role: Role,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let count = Entity::find()
        .filter(Column::Role.eq(role))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .count(conn)
        .await?;

    Ok(count)
}
