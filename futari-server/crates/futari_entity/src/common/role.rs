use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    ToSchema,
    Hash,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "mod")]
    Mod,
    #[sea_orm(string_value = "admin")]
    Admin,
}

impl Role {
    /// Higher value = higher priority (Admin > Mod)
    pub fn display_priority(self) -> u8 {
        match self {
            Self::Mod => 1,
            Self::Admin => 2,
        }
    }
}