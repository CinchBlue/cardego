extern crate askama;
extern crate diesel;
extern crate juniper;

use super::schema::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use diesel::sql_types::{Integer, Text};

// NOTE: Cards also have many-to-one card-attributes that are stored on a
// separate table as per usual data schema normalization.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    juniper::GraphQLObject,
    Identifiable,
    Queryable,
    Insertable,
    QueryableByName,
)]
#[table_name = "cards"]
pub struct Card {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub initiative: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub cardclass: &'a str,
    pub action: &'a str,
    pub speed: &'a str,
    pub initiative: i32,
    pub name: &'a str,
    pub desc: &'a str,
    pub image_url: Option<&'a str>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    juniper::GraphQLObject,
    Identifiable,
    Queryable,
    Insertable,
    QueryableByName,
)]
#[table_name = "decks"]
pub struct Deck {
    pub id: i32,
    pub decktype: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "decks"]
pub struct NewDeck<'a> {
    pub id: Option<i32>,
    pub name: &'a str,
    pub decktype: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject, Queryable)]
pub struct DeckCardRelation {
    pub id: i32,
    pub deck_id: i32,
    pub card_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "decks_cards_relation"]
pub struct NewDeckCardRelation {
    pub deck_id: i32,
    pub card_id: i32,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    juniper::GraphQLObject,
    Identifiable,
    Queryable,
    QueryableByName,
)]
#[table_name = "card_attributes"]
pub struct CardAttribute {
    pub id: i32,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "card_attributes"]
pub struct NewCardAttribute<'a> {
    pub name: &'a str,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject, Identifiable, Queryable)]
#[table_name = "cards_card_attributes_relation"]
pub struct CardCardAttributeRelation {
    pub id: i32,
    pub card_id: i32,
    pub card_attribute_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "cards_card_attributes_relation"]
pub struct NewCardCardAttributeRelation {
    pub card_id: i32,
    pub card_attribute_id: i32,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct CardIdWithCardAttribute {
    #[sql_type = "Integer"]
    pub card_id: i32,
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Integer"]
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct FullCardData {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub initiative: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub attributes: Option<Vec<CardAttribute>>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    juniper::GraphQLObject,
    Identifiable,
    Queryable,
    QueryableByName,
)]
#[table_name = "search_card_data"]
pub struct SearchCardData {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub initiative: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub attribute_ids: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewFullCardData {
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub initiative: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub card_attributes: Option<Vec<i32>>,
}

lazy_static! {
    pub static ref TRAIT_SHORT_TO_FULLNAME: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Kn".to_string(), "Knowledge");
        m.insert("Tr".to_string(), "Trait");
        m.insert("It".to_string(), "Item");
        m.insert("Eq".to_string(), "Equipment");
        m.insert("Ar".to_string(), "Armor");
        m.insert("Co".to_string(), "Consumable");
        m.insert("Te".to_string(), "Technique");
        m.insert("Sp".to_string(), "Spell");
        m.insert("Po".to_string(), "Power");
        m.insert("1H".to_string(), "1-Handed Arms");
        m.insert("2H".to_string(), "2-Handed Arms");

        m
    };
    pub static ref TABLE_TO_UNIQUE_SEARCH_TERMS: HashMap<String, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            "search_card_data".to_string(),
            vec![
                "id",
                "cardclass",
                "action",
                "speed",
                "initiative",
                "name",
                "desc",
                "image_url",
            ],
        );
        m.insert(
            "card_attributes".to_string(),
            vec!["attribute_name", "attribute_id"],
        );

        m
    };
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use crate::models::Card;
    use serde_json::Value;

    #[test]
    fn given_card_json_string_when_serialize_then_successful() {
        let json_string = r#"{"id":1337,"cardclass":"1H","action":"Attack","speed":"Normal","initiative":3,"name":"Lmao","desc":"Range 1. Lmao.","image_url":null}"#;
        let _json_value: Value = serde_json::from_str(json_string).unwrap();
        let _json_card_value: Card = serde_json::from_str(json_string).unwrap();
    }
}
