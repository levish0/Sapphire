use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for verify email request.
pub struct VerifyEmailRequest {
    /// 이메일 인증 토큰
    #[validate(length(min = 1))]
    pub token: String,
}
