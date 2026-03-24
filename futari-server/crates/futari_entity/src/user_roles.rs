use sea_orm::prelude::*;
use uuid::Uuid;

use super::common::Role;
use super::users::Entity as UsersEntity;

/// 사용자-역할 매핑 (다중 역할 지원)
/// Everyone/User는 암시적 역할 (session 유무로 판별), 이 테이블에 저장하지 않음
/// 명시적 역할만 저장: mod, admin
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(not_null)]
    pub role: Role,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub granted_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub expires_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
