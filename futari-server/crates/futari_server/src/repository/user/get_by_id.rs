use sea_orm::{ConnectionTrait, EntityTrait, QuerySelect};
use uuid::Uuid;
use futari_entity::users::{Entity as UserEntity, Model as UserModel};
use futari_errors::errors::Errors;

/// 사용자 ID로 사용자 단건을 조회한다(없으면 에러).
///
/// # 역할
/// ID 단건 조회 후 결과가 없으면 `Errors::UserNotFound`를 반환한다.
///
/// # Errors
/// - 사용자 미존재 시 `Errors::UserNotFound`
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_get_user_by_id<C>(conn: &C, id: Uuid) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find_by_id(id).one(conn).await?;

    user.ok_or(Errors::UserNotFound)
}

/// Get user by id with row-level lock (SELECT ... FOR UPDATE).
/// Used to serialize critical per-user mutations.
pub async fn repository_get_user_by_id_for_update<C>(
    conn: &C,
    id: Uuid,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find_by_id(id)
        .lock_exclusive()
        .one(conn)
        .await?;

    user.ok_or(Errors::UserNotFound)
}
