extern crate diesel;
extern crate askama;

use super::schema::*;
use serde::{Deserialize, Serialize};
use askama::Template;

use std::collections::HashMap;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
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
    pub name: &'a str,
    pub desc: &'a str,
    pub image_url: Option<String>,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
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
#[derive(Queryable, Insertable)]
#[table_name = "decks_cards_relation"]
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


lazy_static! {
    pub static ref TRAIT_SHORT_TO_FULLNAME: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Kn".to_string(), "Knowledge");
        m.insert("Tr".to_string(), "Trait");
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

