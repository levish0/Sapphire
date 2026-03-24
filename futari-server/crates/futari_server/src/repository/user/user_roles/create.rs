use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;
use futari_entity::common::Role;
use futari_entity::user_roles::{ActiveModel, Model};
use futari_errors::errors::Errors;

/// 사용자에게 역할 부여
pub async fn repository_create_user_role<C>(
    conn: &C,
    user_id: Uuid,
    role: Role,
    expires_at: Option<DateTime<Utc>>,
) -> Result<Model, Errors>
where
    C: ConnectionTrait,
{
    let new_role = ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        role: Set(role),
        granted_at: Default::default(),
        expires_at: Set(expires_at),
    };

    let result = new_role.insert(conn).await?;
    Ok(result)
}
