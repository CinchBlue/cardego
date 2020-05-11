#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

extern crate anyhow;

pub mod models;
pub mod schema;
pub mod errors;

use diesel::prelude::*;
use diesel::{SqliteConnection};

use anyhow::{Result};
use log::{info, debug};

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
        
        let result = cards
                .find(card_id)
                .first(self.connection.as_ref())?;

        Ok(result)
    }
    
    pub fn get_cards_by_user_set_name(&self, set_name: String)
        -> Result<Vec<Card>> {
        use self::schema::*;
        
        allow_tables_to_appear_in_same_query!(
            user_sets_to_cards,
            cards,
            user_sets);
       
        // Get the list of cards from the card set
        let query = user_sets::dsl::user_sets
                .inner_join(user_sets_to_cards::dsl::user_sets_to_cards
                        .on(user_sets::dsl::name.like(set_name)))
                .inner_join(cards::dsl::cards
                        .on(user_sets_to_cards::dsl::card_id
                                .eq (cards::dsl::id)))
                .filter(user_sets::dsl::id
                        .eq(user_sets_to_cards::dsl::user_set_id))
                .select(cards::all_columns);
        
        debug!("{}", diesel::debug_query::<diesel::sqlite::Sqlite, _>
            (&query).to_string());
        
        let results = query.load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_user_sets_by_name(&self, s: String)
        -> Result<Vec<UserSet>> {
        use self::schema::user_sets::dsl::*;
    
        let results = user_sets
                .filter(name.like(format!("%{}%", s)))
                .load(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    
    pub fn query_cards_by_name(&self, s: &str) -> Result<Vec<Card>> {
        use self::schema::cards::dsl::*;
        
        let results = cards
                .filter(name.like(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_cardtype(&self, s: &str)
        -> Result<Vec<Card>, Box<dyn Error>> {
        use self::schema::cards::dsl::*;
        
        let results = cards
                .filter(cardtype.eq(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
    
    pub fn query_cards_by_cost(&self, s: &str) -> Result<Vec<Card>, Box<dyn Error>> {
        use crate::schema::cards::columns::cost;
        use crate::schema::cards::dsl::cards;
        
        let results = cards
                .filter(cost.like(s))
                .load::<Card>(self.connection.as_ref())?;
        
        Ok(results)
    }
}
