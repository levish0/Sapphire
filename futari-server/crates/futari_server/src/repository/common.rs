use sea_orm::FromQueryResult;
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QuerySelect, Select};
use futari_errors::errors::Errors;

pub(crate) async fn repository_query_exists<C, E>(
    conn: &C,
    query: Select<E>,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
    E: EntityTrait,
    E::Model: FromQueryResult + Send + Sync,
{
    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}
