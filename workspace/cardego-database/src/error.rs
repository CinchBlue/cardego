use sea_orm::DbErr;
use thiserror::Error;

/// Represents all possible errors that can come from the database API.
///
/// Uses `thiserror` to make the error conversion boilerplate bearable.
#[derive(Error, Debug)]
pub enum APIError {
    #[error("API error due to database error: {0:?}")]
    DatabaseError(#[from] DbErr),
}
