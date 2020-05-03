#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
extern crate anyhow;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::{SqliteConnection};

use self::models::*;

use std::error::{Error};
use anyhow::Result;

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
