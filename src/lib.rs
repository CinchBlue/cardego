#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

extern crate anyhow;

pub mod models;
pub mod schema;
pub mod errors;
pub mod image;

use diesel::prelude::*;
use diesel::{SqliteConnection};

use anyhow::{Result, anyhow};
use log::{debug};

use self::models::*;
use self::errors::*;

use std::error::{Error};

pub struct CardDatabase {
    connection: Box<SqliteConnection>,
}

impl CardDatabase {
    pub fn new(url: &str)
        -> Result<CardDatabase> {
        
        let connection = SqliteConnection::establish(&url)?;
        
        Ok(CardDatabase { connection: Box::new(connection) })
    }
    
    pub fn get_card(&self, card_id: i32) -> Result<Card> {
        use self::schema::cards::dsl::*;
       
        debug!("card_id: {}", card_id);
        
        let result = cards
                .find(card_id)
                .first(self.connection.as_ref())?;
        
        debug!("result found");

        Ok(result)
    }
    
    pub fn put_card(&mut self, card: &Card) -> Result<Card> {
        debug!("put_card: {:?}", card);
        
        use schema::cards;
        use schema::card_attributes;
        use schema::cards_card_attributes_relation;
        
        diesel::replace_into(cards::table)
                .values(card)
                .execute(self.connection.as_mut())?;
        
        // Get the id of the card
        let card_name = &card.name;
        let new_card_result = self.query_cards_by_name_formatted(card_name)?;
       
        let new_card = new_card_result
                .first()
                .into_iter()
                .nth(0)
                .ok_or(ServerError::OtherError(anyhow!(
                "Could not find expected card name '{}' after successful \
                insert/replace into SQLite database.", &card.name)))?;
        let last_id = new_card.id;
        
        
        
        debug!("Put card with id {}", last_id);
        
        
        debug!("put_card succeeded");
        Ok(new_card.clone())
    }
    
    pub fn put_deck(&mut self, name: String, ids: Vec<i32>) -> Result<Deck> {
        debug!("put_deck: {} {:?}", name, ids);
    
        use schema::decks;
        use schema::decks_cards_relation;
   
        // First, insert the deck entry
        let new_deck = NewDeck { id: None, name: &name, decktype: "user" };
    
        diesel::insert_into(decks::table)
                .values(&new_deck)
                //.on_conflict(decks::name)
                //.do_update()
                //.set(&new_deck)
                .execute(self.connection.as_mut())?;
        
        // Get the id of the deck
        let new_deck = self.get_deck_by_name(&name)?;
        let last_id = new_deck.id;
        
        debug!("Created new deck with id {}", last_id);
    
        
        // Then, insert the deck's cards into the deck itself
        let new_deck_card_relations: Vec<NewDeckCardRelation> = ids.iter()
                .map(|card_id|  NewDeckCardRelation { deck_id: last_id,
                    card_id: *card_id, })
                .collect();
        
        diesel::insert_into(decks_cards_relation::table)
                .values(&new_deck_card_relations)
                .execute(self.connection.as_mut())?;
        
        debug!("put_deck succeeded");
        Ok(new_deck)
    }
    
    pub fn get_cards_by_deck_name(&self, set_name: String)
        -> Result<Vec<Card>> {
        use self::schema::*;
        
        allow_tables_to_appear_in_same_query!(
            decks_cards_relation,
            cards,
            decks);
       
        // Get the list of cards from the card set
        let query = decks::dsl::decks
                .inner_join(decks_cards_relation::dsl::decks_cards_relation
                        .on(decks::dsl::name
                                .like(set_name)))
                .inner_join(cards::dsl::cards
                        .on(decks_cards_relation::dsl::card_id
                                .eq(cards::dsl::id)))
                .filter(decks::dsl::id
                        .eq(decks_cards_relation::dsl::deck_id))
                .select(cards::all_columns);
        
        debug!("{}", diesel::debug_query::<diesel::sqlite::Sqlite, _>
            (&query).to_string());
        
        let results = query.load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn get_deck_by_name(&self, s: &str)
        -> Result<Deck> {
        use self::schema::decks::dsl::*;
        
        let result = decks
                .filter(name.like(s))
                .get_result(self.connection.as_ref())?;
        
        debug!("{:?}", result);
        
        Ok(result)
    }
    
    pub fn query_decks_by_name(&self, s: String)
        -> Result<Vec<Deck>> {
        use self::schema::decks::dsl::*;
        
        let results = decks
                .filter(name.like(format!("%{}%", s)))
                .load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_name_formatted(&self, s: &str)
        -> Result<Vec<Card>> {
        use self::schema::cards::dsl::*;
        
        let results = cards
                .filter(name.like(s))
                .load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_name(&self, s: String) -> Result<Vec<Card>> {
        use self::schema::cards::dsl::*;
    
        let results = cards
                .filter(name.like(format!("%{}%", s)))
                .load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_cardclass(&self, s: &str)
        -> Result<Vec<Card>, Box<dyn Error>> {
        use self::schema::cards::dsl::*;
        
        let results = cards
                .filter(cardclass.eq(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_action(&self, s: &str)
        -> Result<Vec<Card>, Box<dyn Error>> {
        use crate::schema::cards::columns::action;
        use crate::schema::cards::dsl::cards;
        
        let results = cards
                .filter(action.like(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn get_card_attributes_by_card_id(&self, card_id: i32)
        -> Result<Vec<CardAttribute>> {
        use self::schema::*;
    
        allow_tables_to_appear_in_same_query!(
            cards_card_attributes_relation,
            cards,
            card_attributes);
    
        // Get the list of card_attributes for a given card
        //
        // SELECT   card_attributes.id AS card_attribute_id,
        //          card_attributes.name AS card_attribute_name,
        //          card_attributes.[order] AS card_attribute_order
        // FROM card_attributes
        // JOIN
        //         (
        //             cards_card_attributes_relation
        //         )
        // ON card_attributes.id = cards_card_attributes_relation.card_attribute_id
        // JOIN
        //         (
        //             cards
        //         )
        // ON cards.id = cards_card_attributes_relation.card_id;
        let query = card_attributes::dsl::card_attributes
                .inner_join(cards_card_attributes_relation::dsl::cards_card_attributes_relation
                        .on(card_attributes::dsl::id
                                .eq(cards_card_attributes_relation::dsl::card_attribute_id)))
                .inner_join(cards::dsl::cards
                        .on(cards::dsl::id
                                .eq(cards_card_attributes_relation::dsl::card_id)))
                .filter(cards::dsl::id
                        .eq(card_id))
                .select(card_attributes::all_columns);
    
        debug!("{}", diesel::debug_query::<diesel::sqlite::Sqlite, _>
                (&query).to_string());
    
        let results = query.load(self.connection.as_ref())?;
    
        Ok(results)
    }
}
