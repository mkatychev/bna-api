//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "city")]
pub struct Model {
    #[sea_orm(unique)]
    pub city_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub country: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub state: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    #[sea_orm(column_type = "Double", nullable)]
    pub latitude: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub longitude: Option<f64>,
    pub region: Option<String>,
    pub state_abbrev: Option<String>,
    pub speed_limit: Option<i32>,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::census::Entity")]
    Census,
    #[sea_orm(has_many = "super::speed_limit::Entity")]
    SpeedLimit,
    #[sea_orm(has_many = "super::summary::Entity")]
    Summary,
}

impl Related<super::census::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Census.def()
    }
}

impl Related<super::speed_limit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SpeedLimit.def()
    }
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
