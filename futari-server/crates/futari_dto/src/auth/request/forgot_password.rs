use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for forgot password request.
pub struct ForgotPasswordRequest {
    /// 비밀번호 재설정을 요청할 이메일 주소
    #[validate(email)]
    pub email: String,
}
