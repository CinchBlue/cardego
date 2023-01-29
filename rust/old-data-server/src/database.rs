extern crate diesel;

use crate::diesel::Connection;
use diesel::prelude::SqliteConnection;

// NOTE: do not use r2d2 with SQLite + Diesel because SQLite's lack of
// support for batched inserts is currently causing compilation errors. Just
// don't use connection pooling until we swap to MySQL or PostgreSQL.
pub struct DatabaseContext {
    pub connection: Box<SqliteConnection>,
}

impl DatabaseContext {
    pub fn new(url_endpoint: &str) -> anyhow::Result<DatabaseContext> {
        let connection = SqliteConnection::establish(url_endpoint)?;

        Ok(Self {
            connection: Box::new(connection),
        })
    }
}
