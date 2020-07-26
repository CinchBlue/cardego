#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

extern crate anyhow;

pub mod models;
pub mod schema;
pub mod errors;
pub mod image;

use diesel::prelude::*;
use diesel::{SqliteConnection};

use anyhow::{Result};
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
                        .on(decks::dsl::name.like(set_name)))
                .inner_join(cards::dsl::cards
                        .on(decks_cards_relation::dsl::card_id
                                .eq (cards::dsl::id)))
                .filter(decks::dsl::id
                        .eq(decks_cards_relation::dsl::deck_id))
                .select(cards::all_columns);
        
        debug!("{}", diesel::debug_query::<diesel::sqlite::Sqlite, _>
            (&query).to_string());
        
        let results = query.load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_decks_by_name(&self, s: String)
        -> Result<Vec<Deck>> {
        use self::schema::decks::dsl::*;
        
        let results = decks
                .filter(name.like(format!("%{}%", s)))
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
    
    pub fn query_cards_by_action(&self, s: &str) -> Result<Vec<Card>, Box<dyn
    Error>> {
        use crate::schema::cards::columns::action;
        use crate::schema::cards::dsl::cards;
        
        let results = cards
                .filter(action.like(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
}
