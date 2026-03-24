use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use futari_entity::users::{Column as UserColumn, Entity as UserEntity, Model as UserModel};
use futari_errors::errors::Errors;

/// 사용자 ID 목록으로 사용자들을 조회한다.
///
/// # 역할
/// 빈 입력이면 빈 벡터를 즉시 반환하고, 아니면 `IN` 조건 조회를 수행한다.
///
/// # Errors
/// - 조회 실패 시 DB/저장소 에러를 반환한다.
pub async fn repository_find_users_by_ids<C>(
    conn: &C,
    ids: &[Uuid],
) -> Result<Vec<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let users = UserEntity::find()
        .filter(UserColumn::Id.is_in(ids.to_vec()))
        .all(conn)
        .await?;

    Ok(users)
}
