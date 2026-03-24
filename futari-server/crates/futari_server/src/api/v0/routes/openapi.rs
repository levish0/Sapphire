use super::action_logs::openapi::ActionLogsOpenApi;
use super::auth::openapi::AuthApiDoc;
use super::discussion::openapi::DiscussionApiDoc;
use super::document::edit_request::openapi::EditRequestApiDoc;
use super::document::openapi::DocumentApiDoc;
use super::eventstream::openapi::EventStreamOpenApi;
use super::moderation::openapi::ModerationApiDoc;
use super::notification::openapi::NotificationApiDoc;
use super::points::openapi::PointsApiDoc;
use super::reports::openapi::ReportsApiDoc;
use super::search::openapi::SearchApiDoc;
use super::user::openapi::UserApiDoc;
use super::user_preferences::openapi::UserPreferencesApiDoc;
use super::watchlist::openapi::WatchlistApiDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi()]
pub struct V0ApiDoc;

impl V0ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(AuthApiDoc::openapi());
        openapi.merge(UserApiDoc::openapi());
        openapi.merge(SearchApiDoc::openapi());
        openapi.merge(DocumentApiDoc::openapi());
        openapi.merge(DiscussionApiDoc::openapi());
        openapi.merge(EditRequestApiDoc::openapi());
        openapi.merge(NotificationApiDoc::openapi());
        openapi.merge(WatchlistApiDoc::openapi());
        openapi.merge(ModerationApiDoc::openapi());

        openapi.merge(ActionLogsOpenApi::openapi());
        openapi.merge(ReportsApiDoc::openapi());
        openapi.merge(UserPreferencesApiDoc::openapi());
        openapi.merge(EventStreamOpenApi::openapi());
        openapi.merge(PointsApiDoc::openapi());
        openapi
    }
}
