use sea_orm::prelude::*;
use uuid::Uuid;

use super::user_oauth_connections::Entity as UserOAuthConnectionsEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", not_null)]
    pub display_name: String,
    #[sea_orm(column_type = "Text", not_null, unique)]
    pub handle: String, // Unique
    #[sea_orm(column_type = "Text", nullable)]
    pub bio: Option<String>,
    #[sea_orm(string_len = 254, not_null, unique)]
    pub email: String, // Unique
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub banner_image: Option<String>,
    // TOTP 2FA
    /// TOTP secret (Base32 인코딩)
    #[sea_orm(column_type = "Text", nullable)]
    pub totp_secret: Option<String>,
    /// TOTP 활성화 시각 (None = 비활성화)
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub totp_enabled_at: Option<DateTimeUtc>,
    /// TOTP 백업 코드 (10개, 평문)
    #[sea_orm(nullable)]
    pub totp_backup_codes: Option<Vec<String>>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "UserOAuthConnectionsEntity")]
    OAuthConnections,
}

impl Related<UserOAuthConnectionsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::OAuthConnections.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
