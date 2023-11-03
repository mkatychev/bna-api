//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "opportunity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub bna_uuid: Uuid,
    #[sea_orm(column_type = "Double")]
    pub employment: f64,
    #[sea_orm(column_type = "Double")]
    pub higher_education: f64,
    #[sea_orm(column_type = "Double")]
    pub k12_education: f64,
    #[sea_orm(column_type = "Double")]
    pub score: f64,
    #[sea_orm(column_type = "Double")]
    pub technical_vocational_college: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::summary::Entity",
        from = "Column::BnaUuid",
        to = "super::summary::Column::BnaUuid",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Summary,
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
