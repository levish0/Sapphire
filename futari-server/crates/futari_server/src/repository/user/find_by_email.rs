use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use futari_entity::users::{Column as UsersColumn, Entity as UserEntity, Model as UserModel};
use futari_errors::errors::Errors;

/// 이메일로 사용자 존재 여부를 조회한다.
///
/// # 역할
/// `email` 조건 단건 조회를 수행하고 `Option<UserModel>`을 반환한다.
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_find_user_by_email<C>(
    conn: &C,
    email: String,
) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find()
        .filter(UsersColumn::Email.eq(email))
        .one(conn)
        .await?;

    Ok(user)
}
