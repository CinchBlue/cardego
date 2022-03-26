pub mod error;
pub mod models;
pub mod shapes;

use error::APIError;
use log::debug;
use models::{attributes, cards};
use sea_orm::{Database, DatabaseConnection, DatabaseTransaction};

type APIResult<T> = Result<T, APIError>;

/// An API connection object that abstracts away the database.
pub struct APIConnection {
    pub db: DatabaseConnection,
}

impl APIConnection {
    /// Connect to the database.
    pub async fn connect() -> APIResult<Self> {
        let conn_url = "sqlite://./runtime/data/databases/cards-ymir.db";
        let db = {
            debug!("try to connect to {}", conn_url);
            Database::connect(conn_url).await?
        };
        Ok(APIConnection { db })
    }
}

pub struct APIOperation {
    pub db_txn: DatabaseTransaction,
}

pub struct QueryCardsInput {
    pub ids: Option<Vec<String>>,
    pub name_regex: Option<String>,
    pub desc_regex: Option<String>,
    pub image_url_regex: Option<String>,
}

impl APIOperation {
    // Query for cards
    pub async fn query_cards(
        query: QueryCardsInput,
    ) -> APIResult<Vec<(cards::Entity, Vec<attributes::Entity>)>> {
        // find and filter
        todo!()
    }
}
