extern crate diesel;

use super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub cardclass: String,
    pub action: String,
    pub speed: String,
    pub name: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub cardclass: &'a str,
    pub action: &'a str,
    pub speed: &'a str,
    pub name: &'a str,
    pub desc: &'a str,
}


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Deck {
    pub id: i32,
    pub decktype: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "decks"]
pub struct NewDeck<'a> {
    pub name: &'a str,
    pub decktype: &'a str,
}


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct DeckCardRelation {
    pub id: i32,
    pub deck_id: i32,
    pub card_id: i32,
}


#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "decks_cards_relation"]
pub struct NewDeckCardRelation {
    pub deck_id: i32,
    pub card_id: i32,
}
