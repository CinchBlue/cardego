//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cards")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cardsets_to_cards::Entity")]
    CardsetsToCards,
    #[sea_orm(has_many = "super::cards_to_attributes::Entity")]
    CardsToAttributes,
}

impl Related<super::cardsets_to_cards::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardsetsToCards.def()
    }
}

impl Related<super::cards_to_attributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardsToAttributes.def()
    }
}

impl Related<super::attributes::Entity> for Entity {
    fn to() -> RelationDef {
        super::cards_to_attributes::Relation::Attributes.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::cards_to_attributes::Relation::Attributes.def().rev())
    }
}
pub struct CardsToAttributes;

impl Linked for CardsToAttributes {
    type FromEntity = Entity;
    type ToEntity = super::attributes::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::cards_to_attributes::Relation::Cards.def().rev(),
            super::cards_to_attributes::Relation::Attributes.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
