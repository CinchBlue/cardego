extern crate diesel;
extern crate askama;

use super::schema::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;


// NOTE: Cards also have many-to-one card-attributes that are stored on a
// separate table as per usual data schema normalization.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable, Insertable)]
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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub cardclass: &'a str,
    pub action: &'a str,
    pub speed: &'a str,
    pub initiative: i32,
    pub name: &'a str,
    pub desc: &'a str,
    pub image_url: Option<String>,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable)]
#[derive(Insertable)]
#[table_name = "decks"]
pub struct Deck {
    pub id: i32,
    pub decktype: String,
    pub name: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "decks"]
pub struct NewDeck<'a> {
    pub id: Option<i32>,
    pub name: &'a str,
    pub decktype: &'a str,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct DeckCardRelation {
    pub id: i32,
    pub deck_id: i32,
    pub card_id: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "decks_cards_relation"]
pub struct NewDeckCardRelation {
    pub deck_id: i32,
    pub card_id: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable)]
pub struct CardAttribute {
    pub id: i32,
    pub name: String,
    pub order: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "card_attributes"]
pub struct NewCardAttribute<'a> {
    pub name: &'a str,
    pub order: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable)]
#[table_name = "cards_card_attributes_relation"]
pub struct CardCardAttributeRelation {
    pub id: i32,
    pub card_id: i32,
    pub card_attribute_id: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "cards_card_attributes_relation"]
pub struct NewCardCardAttributeRelation {
    pub card_id: i32,
    pub card_attribute_id: i32,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct FullCardData {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub initiative: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub card_attributes: Vec<CardAttribute>,
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
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    
    use serde_json::Value;
    use crate::models::Card;
    
    #[test]
    fn given_card_json_string_when_serialize_then_successful() {
        let json_string = r#"{"id":1337,"cardclass":"1H","action":"Attack","speed":"Normal","initiative":3,"name":"Lmao","desc":"Range 1. Lmao.","image_url":null}"#;
        let json_value: Value = serde_json::from_str(json_string).unwrap();
        let json_card_value: Card = serde_json::from_str(json_string).unwrap();
    }
}

