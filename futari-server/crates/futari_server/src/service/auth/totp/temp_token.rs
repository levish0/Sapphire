use crate::utils::redis_cache::{get_optional_json_and_delete, set_json_with_ttl};
use chrono::{DateTime, Utc};
use rand::Rng;
use redis::aio::ConnectionManager as RedisClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use futari_errors::errors::Errors;

const TEMP_TOKEN_TTL_SECONDS: u64 = 120; // 2분

/// TOTP 검증용 임시 토큰 (Redis 저장용)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpTempToken {
    pub token: String,
    pub user_id: Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub remember_me: bool,
    pub created_at: DateTime<Utc>,
}

impl TotpTempToken {
    /// TOTP 2차 인증용 임시 토큰 객체를 생성한다.
    ///
    /// # 역할
    /// 256-bit 랜덤 토큰과 요청 컨텍스트를 메모리 객체로 구성한다.
    pub fn new(
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
        remember_me: bool,
    ) -> Self {
        // 암호학적으로 안전한 랜덤 토큰 생성 (32 bytes = 256 bits)
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        let token = hex::encode(bytes);

        Self {
            token,
            user_id,
            user_agent,
            ip_address,
            remember_me,
            created_at: Utc::now(),
        }
    }

    /// 임시 토큰 저장용 Redis 키를 생성한다.
    pub fn redis_key(&self) -> String {
        format!("totp_temp:{}", self.token)
    }

    /// 임시 토큰 생성 및 Redis 저장
    pub async fn create(
        redis: &RedisClient,
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
        remember_me: bool,
    ) -> Result<Self, Errors> {
        let temp_token = Self::new(user_id, user_agent, ip_address, remember_me);

        set_json_with_ttl(
            redis,
            &temp_token.redis_key(),
            &temp_token,
            TEMP_TOKEN_TTL_SECONDS,
        )
        .await?;

        Ok(temp_token)
    }

    /// 임시 토큰 조회 및 삭제 (일회용)
    pub async fn get_and_delete(redis: &RedisClient, token: &str) -> Result<Option<Self>, Errors> {
        let key = format!("totp_temp:{}", token);

        // GETDEL: 조회 + 삭제 원자적 수행
        get_optional_json_and_delete(redis, &key, |e| {
            Errors::SysInternalError(format!("TOTP temp token deserialization failed: {}", e))
        })
        .await
    }
}
