use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use futari_entity::users::{Column as UsersColumn, Entity as UserEntity, Model as UserModel};
use futari_errors::errors::Errors;

/// 이메일로 사용자 단건을 조회한다(없으면 에러).
///
/// # 역할
/// `email` 조건 단건 조회 후 결과가 없으면 `Errors::UserNotFound`를 반환한다.
///
/// # Errors
/// - 사용자 미존재 시 `Errors::UserNotFound`
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_get_user_by_email<C>(conn: &C, email: String) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find()
        .filter(UsersColumn::Email.eq(email))
        .one(conn)
        .await?;

    user.ok_or(Errors::UserNotFound)
}
