use super::common::ISSUER;
use crate::repository::user::{
    UserUpdateParams, repository_get_user_by_id, repository_update_user,
};
use rand::RngExt;
use sea_orm::{DatabaseConnection, TransactionTrait};
use totp_rs::{Algorithm, Secret, TOTP};
use tracing::info;
use uuid::Uuid;
use futari_dto::auth::response::TotpSetupResponse;
use futari_errors::errors::{Errors, ServiceResult};

/// TOTP 설정 시작: secret 생성, DB 저장 (아직 활성화 안 함), QR 반환
pub async fn service_totp_setup(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<TotpSetupResponse> {
    let txn = db.begin().await?;

    // 사용자 조회
    let user = repository_get_user_by_id(&txn, user_id).await?;

    // 이미 TOTP 활성화된 경우
    if user.totp_enabled_at.is_some() {
        return Err(Errors::TotpAlreadyEnabled);
    }

    // Secret 생성 (20 bytes = 160 bits, RFC 4226 권장)
    let (secret_bytes, secret_base32) = {
        let mut rng = rand::rng();
        let bytes: [u8; 20] = rng.random();
        let secret = Secret::Raw(bytes.to_vec());
        (bytes, secret.to_encoded().to_string())
    };

    // TOTP 객체 생성
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,  // digits
        1,  // skew
        30, // step
        secret_bytes.to_vec(),
        Some(ISSUER.to_string()),
        user.email,
    )
    .map_err(|_| Errors::TotpSecretGenerationFailed)?;

    // QR 코드 생성 (PNG base64)
    let qr_code_uri = totp.get_url();
    let qr_code_png_base64 = totp
        .get_qr_base64()
        .map_err(|_| Errors::TotpQrGenerationFailed)?;

    // DB에 secret 저장 (totp_enabled_at은 아직 NULL)
    repository_update_user(
        &txn,
        user_id,
        UserUpdateParams {
            totp_secret: Some(Some(secret_base32)),
            ..Default::default()
        },
    )
    .await?;

    txn.commit().await?;

    info!(user_id = %user_id, "TOTP setup initiated");

    Ok(TotpSetupResponse {
        qr_code_base64: qr_code_png_base64,
        qr_code_uri,
    })
}
