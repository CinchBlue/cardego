#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate nom;

extern crate anyhow;

pub mod database;
pub mod errors;
pub mod image;
pub mod models;
pub mod schema;
pub mod search;

use diesel::prelude::*;

use anyhow::{anyhow, Result};
use log::debug;

use self::database::DatabaseContext;
use self::errors::*;
use self::models::*;

use std::error::Error;

pub struct ServerState {
    pub config: ApplicationConfig,
    pub schema: crate::search::Schema,
}

pub struct ApplicationConfig {
    pub database_endpoint: String,
}

impl ApplicationConfig {
    pub fn new() -> anyhow::Result<Self> {
        debug!("Initializing ApplicationConfig");

        Ok(Self {
            database_endpoint: String::from("runtime/data/databases/cards.db"),
        })
    }
}

impl DatabaseContext {
    pub fn get_card(&self, card_id: i32) -> Result<Card> {
        use self::schema::cards::dsl::*;

        debug!("card_id: {}", card_id);

        let result = cards.find(card_id).first(self.connection.as_ref())?;

        debug!("result found");

        Ok(result)
    }

    pub fn get_full_card_data(&self, card_id: i32) -> Result<FullCardData> {
        let card = self.get_card(card_id)?;

        let card_attributes = self
            .get_card_attributes_by_card_id(card_id)
            .map(|v| Some(v))
            .unwrap_or(None);

        Ok(FullCardData {
            id: card.id,
            cardclass: card.cardclass,
            action: card.action,
            speed: card.speed,
            initiative: card.initiative,
            name: card.name,
            desc: card.desc,
            image_url: card.image_url,
            card_attributes,
        })
    }

    // TODO: memory management on this needs to be optimized; currently just
    // clone()-ing things like a madman.
    pub fn create_card(&mut self, card_data: &NewFullCardData) -> Result<FullCardData> {
        debug!("create_card: {:?}", card_data);

        use schema::cards;
        use schema::cards_card_attributes_relation;

        let card = NewCard {
            cardclass: &card_data.cardclass,
            action: &card_data.action,
            speed: &card_data.speed,
            initiative: card_data.initiative,
            name: &card_data.name,
            desc: &card_data.desc,
            image_url: card_data.image_url.as_ref().map(|s| s.as_str()),
        };

        diesel::replace_into(cards::table)
            .values(&card)
            .execute(self.connection.as_mut())?;

        // Get the id of the card by querying for the name.
        //
        // TODO: This is very bad and needed to be made deterministic. Cards
        // with duplicate names will destroy this.
        let card_name = &card.name;
        let new_card_result = self.query_cards_by_name_formatted(card_name)?;

        let new_card =
            new_card_result
                .first()
                .into_iter()
                .nth(0)
                .ok_or(ServerError::OtherError(anyhow!(
                    "Could not find expected card name '{}' after successful \
                insert/replace into SQLite database.",
                    &card.name
                )))?;
        let last_id = new_card.id;

        debug!("Created card with id {}", last_id);

        // Insert attributes into the attribute table
        let new_card_attribute_relations: Option<Vec<NewCardCardAttributeRelation>> =
            card_data.card_attributes.as_ref().map(|v| {
                v.iter()
                    .map(|attr| NewCardCardAttributeRelation {
                        card_id: last_id,
                        card_attribute_id: *attr,
                    })
                    .collect()
            });

        match new_card_attribute_relations {
            Some(ref v) => {
                diesel::insert_into(cards_card_attributes_relation::table)
                    .values(v)
                    .execute(self.connection.as_mut())?;

                debug!(
                    "Created card_attributes with ids {:?}",
                    new_card_attribute_relations
                );
            }
            None => {
                debug!("No card attributes to be written; skipping");
            }
        };

        // Get the associated attributes out again
        let card_attributes = self
            .get_card_attributes_by_card_id(last_id)
            .map(|v| Some(v))
            .unwrap_or(None);

        debug!("create_card succeeded");
        Ok(FullCardData {
            id: last_id,
            card_attributes: card_attributes,
            cardclass: card_data.cardclass.clone(),
            action: card_data.action.clone(),
            speed: card_data.speed.clone(),
            initiative: card_data.initiative,
            name: card_data.name.clone(),
            desc: card_data.desc.clone(),
            image_url: card_data.image_url.clone(),
        })
    }

    // TODO: memory management on this needs to be optimized; currently just
    // clone()-ing things like a madman.
    pub fn update_card(&mut self, card_data: FullCardData) -> Result<FullCardData> {
        debug!("update_card: {:?}", card_data);

        use schema::cards;
        use schema::cards_card_attributes_relation;

        let card = Card {
            id: card_data.id,
            cardclass: card_data.cardclass,
            action: card_data.action,
            speed: card_data.speed,
            initiative: card_data.initiative,
            name: card_data.name,
            desc: card_data.desc,
            image_url: card_data.image_url,
        };

        diesel::replace_into(cards::table)
            .values(&card)
            .execute(self.connection.as_mut())?;

        // Get the id of the card by querying for the name.
        //
        // TODO: This is very bad and needed to be made deterministic. Cards
        // with duplicate names will destroy this.
        let card_name = &card.name;
        let new_card_result = self.query_cards_by_name_formatted(card_name)?;

        let new_card =
            new_card_result
                .first()
                .into_iter()
                .nth(0)
                .ok_or(ServerError::OtherError(anyhow!(
                    "Could not find expected card name '{}' after successful \
                insert/replace into SQLite database.",
                    &card.name
                )))?;
        let last_id = new_card.id;

        debug!("Updated card with id {}", last_id);

        // Insert attributes into the attribute table
        let new_card_attribute_relations: Option<Vec<NewCardCardAttributeRelation>> =
            card_data.card_attributes.as_ref().map(|v| {
                v.iter()
                    .map(|attr| NewCardCardAttributeRelation {
                        card_id: last_id,
                        card_attribute_id: attr.id,
                    })
                    .collect()
            });

        match new_card_attribute_relations {
            Some(ref v) => {
                diesel::replace_into(cards_card_attributes_relation::table)
                    .values(v)
                    .execute(self.connection.as_mut())?;

                debug!(
                    "Updated card_attributes with ids {:?}",
                    new_card_attribute_relations
                );
            }
            None => {
                debug!("No card attributes to be written; skipping");
            }
        };

        debug!("update_card succeeded");
        Ok(FullCardData {
            id: last_id,
            card_attributes: card_data.card_attributes.clone(),
            cardclass: card.cardclass,
            action: card.action,
            speed: card.speed,
            initiative: card.initiative,
            name: card.name,
            desc: card.desc,
            image_url: card.image_url,
        })
    }

    pub fn create_deck(&mut self, name: String, ids: Vec<i32>) -> Result<Deck> {
        debug!("put_deck: {} {:?}", name, ids);

        use schema::decks;
        use schema::decks_cards_relation;

        // First, insert the deck entry
        let new_deck = NewDeck {
            id: None,
            name: &name,
            decktype: "user",
        };

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
        let new_deck_card_relations: Vec<NewDeckCardRelation> = ids
            .iter()
            .map(|card_id| NewDeckCardRelation {
                deck_id: last_id,
                card_id: *card_id,
            })
            .collect();

        diesel::insert_into(decks_cards_relation::table)
            .values(&new_deck_card_relations)
            .execute(self.connection.as_mut())?;

        debug!("put_deck succeeded");
        Ok(new_deck)
    }

    pub fn get_cards_by_deck_name(&self, set_name: String) -> Result<Vec<Card>> {
        use self::schema::*;

        allow_tables_to_appear_in_same_query!(decks_cards_relation, cards, decks);

        // Get the list of cards from the card set
        let query = decks::dsl::decks
            .inner_join(
                decks_cards_relation::dsl::decks_cards_relation.on(decks::dsl::name.like(set_name)),
            )
            .inner_join(cards::dsl::cards.on(decks_cards_relation::dsl::card_id.eq(cards::dsl::id)))
            .filter(decks::dsl::id.eq(decks_cards_relation::dsl::deck_id))
            .select(cards::all_columns);

        debug!(
            "{}",
            diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
        );

        let results = query.load(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn get_deck_by_name(&self, s: &str) -> Result<Deck> {
        use self::schema::decks::dsl::*;

        let result = decks
            .filter(name.like(s))
            .get_result(self.connection.as_ref())?;

        debug!("{:?}", result);

        Ok(result)
    }

    pub fn query_decks_by_name(&self, s: String) -> Result<Vec<Deck>> {
        use self::schema::decks::dsl::*;

        let results = decks
            .filter(name.like(format!("%{}%", s)))
            .load(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn query_cards_by_name_formatted(&self, s: &str) -> Result<Vec<Card>> {
        use self::schema::cards::dsl::*;

        let results = cards.filter(name.like(s)).load(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn query_cards_by_name(&self, s: String) -> Result<Vec<Card>> {
        use self::schema::cards::dsl::*;

        let results = cards
            .filter(name.like(format!("%{}%", s)))
            .load(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn query_cards_by_cardclass(&self, s: &str) -> Result<Vec<Card>, Box<dyn Error>> {
        use self::schema::cards::dsl::*;

        let results = cards
            .filter(cardclass.eq(s))
            .load::<Card>(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn query_cards_by_action(&self, s: &str) -> Result<Vec<Card>, Box<dyn Error>> {
        use crate::schema::cards::columns::action;
        use crate::schema::cards::dsl::cards;

        let results = cards
            .filter(action.like(s))
            .load::<Card>(self.connection.as_ref())?;

        Ok(results)
    }

    pub fn get_card_attributes_by_card_id(&self, card_id: i32) -> Result<Vec<CardAttribute>> {
        use self::schema::*;

        allow_tables_to_appear_in_same_query!(
            cards_card_attributes_relation,
            cards,
            card_attributes
        );

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
            .inner_join(
                cards_card_attributes_relation::dsl::cards_card_attributes_relation
                    .on(card_attributes::dsl::id
                        .eq(cards_card_attributes_relation::dsl::card_attribute_id)),
            )
            .inner_join(
                cards::dsl::cards
                    .on(cards::dsl::id.eq(cards_card_attributes_relation::dsl::card_id)),
            )
            .filter(cards::dsl::id.eq(card_id))
            .select(card_attributes::all_columns);

        debug!(
            "{}",
            diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
        );

        let results = query.load(self.connection.as_ref())?;

        Ok(results)
    }
}
