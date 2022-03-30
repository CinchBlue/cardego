pub mod error;
pub mod shapes;

use std::collections::HashMap;

use cardego_data_model::models::{attributes, cards};
use error::APIError;
use log::debug;
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseConnection, DatabaseTransaction, EntityTrait,
    ModelTrait, QueryFilter, TransactionTrait,
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
        let conn_url = "sqlite://./runtime/data/databases/cards-ymir.db";
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

impl APIOperation {
    pub async fn commit(self) -> APIResult<()> {
        Ok(self.db_txn.commit().await?)
    }

    pub async fn rollback(self) -> APIResult<()> {
        Ok(self.db_txn.rollback().await?)
    }

    // Query for cards
    pub async fn query_cards(&self, params: QueryCardsInput) -> APIResult<Vec<shapes::FullCard>> {
        // These are rows of cards with their attributes
        let cards_attr_pairs: Vec<(cards::Model, Option<attributes::Model>)> = async {
            let mut cards_query = cards::Entity::find().find_also_linked(cards::CardsToAttributes);

            if let Some(ids) = params.ids {
                cards_query = cards_query.filter(cards::Column::Id.is_in(ids));
            }

            if let Some(name_regex) = params.name_regex {
                cards_query = cards_query.filter(cards::Column::Name.like(&name_regex));
            }

            if let Some(desc_regex) = params.desc_regex {
                cards_query = cards_query.filter(cards::Column::Desc.like(&desc_regex));
            }

            if let Some(image_url_regex) = params.image_url_regex {
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
}
