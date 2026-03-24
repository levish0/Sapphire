use super::GithubProvider;
use crate::service::oauth::generate_oauth_url::service_generate_oauth_url;
use futari_dto::oauth::request::OAuthAuthorizeFlow;
use futari_dto::oauth::response::OAuthUrlResponse;
use futari_entity::common::OAuthProvider;
use futari_errors::errors::ServiceResult;
use redis::aio::ConnectionManager;

pub async fn service_generate_github_oauth_url(
    redis_conn: &ConnectionManager,
    anonymous_user_id: &str,
    flow: OAuthAuthorizeFlow,
) -> ServiceResult<OAuthUrlResponse> {
    service_generate_oauth_url::<GithubProvider>(
        redis_conn,
        anonymous_user_id,
        flow,
        OAuthProvider::Github,
    )
    .await
}
