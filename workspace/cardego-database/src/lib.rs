pub mod error;
pub mod shapes;

use std::collections::HashMap;

use cardego_data_model::models::{attributes, cards, cards_to_attributes};
use error::APIError;
use log::debug;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::shapes::FullCard;

type APIResult<T> = Result<T, APIError>;

/// An API connection object that abstracts away the database.
pub struct APIConnection {
    pub db_conn: DatabaseConnection,
}

impl APIConnection {
    /// Connect to the database.
    pub async fn connect() -> APIResult<Self> {
        //let conn_url = "sqlite://./runtime/data/databases/cards-ymir.db";
        let conn_url = "postgres://postgres:password@cardego-alpha-ymir.c1qfkettokwq.us-west-2.rds.amazonaws.com/cardego-ymir";
        let db_conn = {
            debug!("try to connect to {}", conn_url);
            Database::connect(conn_url).await?
        };
        Ok(APIConnection { db_conn })
    }

    pub async fn operation(&self) -> APIResult<APIOperation> {
        Ok(APIOperation {
            db_txn: self.db_conn.begin().await?,
        })
    }
}

pub struct APIOperation {
    pub db_txn: DatabaseTransaction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCardsInput {
    pub ids: Option<Vec<String>>,
    pub name_regex: Option<String>,
    pub desc_regex: Option<String>,
    pub image_url_regex: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCardInput {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub attribute_ids: Option<Vec<String>>,
}

impl APIOperation {
    pub async fn commit(self) -> APIResult<()> {
        Ok(self.db_txn.commit().await?)
    }

    pub async fn rollback(self) -> APIResult<()> {
        Ok(self.db_txn.rollback().await?)
    }

    // Query for cards
    pub async fn query_cards(&self, args: QueryCardsInput) -> APIResult<Vec<shapes::FullCard>> {
        // These are rows of cards with their attributes
        let cards_attr_pairs: Vec<(cards::Model, Option<attributes::Model>)> = async {
            let mut cards_query = cards::Entity::find().find_also_linked(cards::CardsToAttributes);

            if let Some(ids) = args.ids {
                cards_query = cards_query.filter(cards::Column::Id.is_in(ids));
            }

            if let Some(name_regex) = args.name_regex {
                cards_query = cards_query.filter(cards::Column::Name.like(&name_regex));
            }

            if let Some(desc_regex) = args.desc_regex {
                cards_query = cards_query.filter(cards::Column::Desc.like(&desc_regex));
            }

            if let Some(image_url_regex) = args.image_url_regex {
                cards_query = cards_query.filter(cards::Column::ImageUrl.like(&image_url_regex));
            }

            cards_query.all(&self.db_txn).await
        }
        .await?;

        // Do a funky collect into a cards vec + attribute map.
        // Hopefully the into_iter() optimizes for memory usage while iterating
        let (cards, mut cards_to_attributes) = cards_attr_pairs.into_iter().fold(
            (
                Vec::<cards::Model>::new(),
                HashMap::<i32, Vec<attributes::Model>>::new(),
            ),
            |(mut cards_vec, mut attr_map), (card, maybe_attr)| {
                if let Some(attr) = maybe_attr {
                    if let Some(attr_vec) = attr_map.get_mut(&card.id) {
                        attr_vec.push(attr.clone());
                    } else {
                        attr_map.insert(card.id, vec![attr.clone()]);
                        cards_vec.push(card);
                    }
                }
                (cards_vec, attr_map)
            },
        );

        // Map each card to their attributes and return the full model.
        let full_cards: Vec<FullCard> = cards
            .into_iter()
            .map(|card| FullCard {
                id: card.id,
                name: card.name,
                desc: card.desc,
                image_url: card.image_url,
                attributes: cards_to_attributes.remove(&card.id).unwrap_or(Vec::new()),
            })
            .collect();

        Ok(full_cards)
    }

    pub async fn update_card(&self, args: UpdateCardInput) -> APIResult<()> {
        let mut found_card = cards::Entity::find_by_id(args.id).one(&self.db_txn).await?;

        match found_card {
            Some(card) => {
                let card_id = card.id;
                let mut card_model: cards::ActiveModel = card.into();

                card_model.name = Set(args.name);
                card_model.desc = Set(args.desc);
                card_model.image_url = Set(args.image_url);

                card_model.update(&self.db_txn).await?;

                if let Some(attributes_ids) = args.attribute_ids {
                    let attribute_models = attributes_ids
                        .into_iter()
                        .map(|attribute_id| cards_to_attributes::ActiveModel {
                            card_id: Set(card_id),
                            attribute_id: Set(attribute_id),
                        })
                        .collect::<Vec<_>>();
                }
                return Ok(());
            }
            None => {
                return Err(APIError::ItemNotFound(
                    "cards".to_owned(),
                    format!("{:?}", args.id),
                ))
            }
        }
    }
}
