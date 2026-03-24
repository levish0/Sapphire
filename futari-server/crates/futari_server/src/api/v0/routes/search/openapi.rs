use utoipa::OpenApi;
use futari_dto::search::{
    DiscussionSearchHit, DiscussionSortField, DocumentSearchHit, SearchDiscussionsRequest,
    SearchDiscussionsResponse, SearchDocumentsRequest, SearchDocumentsResponse, SearchUsersRequest,
    SearchUsersResponse, SortOrder, UserSearchHit,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::search_discussions::search_discussions,
        super::search_documents::search_documents,
        super::search_users::search_users,
    ),
    components(
        schemas(
            SearchDocumentsRequest,
            SearchDocumentsResponse,
            DocumentSearchHit,
            SearchDiscussionsRequest,
            SearchDiscussionsResponse,
            DiscussionSearchHit,
            DiscussionSortField,
            SortOrder,
            SearchUsersRequest,
            SearchUsersResponse,
            UserSearchHit,
        )
    ),
    tags(
        (name = "Search", description = "Search endpoints")
    )
)]
pub struct SearchApiDoc;
