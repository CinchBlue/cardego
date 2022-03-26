//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "attributes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub text_ordering: i32,
    pub name: String,
    pub desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cards_to_attributes::Entity")]
    CardsToAttributes,
}

impl Related<super::cards_to_attributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardsToAttributes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
