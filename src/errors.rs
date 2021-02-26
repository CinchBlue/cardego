extern crate anyhow;
extern crate derive_more;
extern crate thiserror;

use actix_web::http::StatusCode;
use anyhow::anyhow;
use std::convert::From;

pub type Result<T> = std::result::Result<T, crate::AppError>;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("DatabaseConnectionError: Could not conenct to database")]
    DatabaseConnectionError,
    #[error("FileIOError: file: {0}")]
    FileIOError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Requested resource not found")]
    ResourceNotFound,
    #[error("Invalid input for operation found: {0}")]
    InvalidInput(String),
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Server error: `{0}`")]
    Server(ServerError),
    #[error("Client error: `{0}`")]
    Client(ClientError),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Server(ServerError::IOError(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Server(ServerError::OtherError(err))
    }
}

impl From<AppError> for std::io::Error {
    fn from(err: AppError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err)
    }
}

impl From<ServerError> for crate::AppError {
    fn from(err: ServerError) -> Self {
        AppError::Server(err)
    }
}

impl From<ServerError> for std::io::Error {
    fn from(err: ServerError) -> Self {
        std::io::Error::from(AppError::from(err))
    }
}

impl From<ClientError> for crate::AppError {
    fn from(err: ClientError) -> Self {
        AppError::Client(err)
    }
}

impl From<ClientError> for std::io::Error {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::ResourceNotFound => std::io::Error::new(std::io::ErrorKind::NotFound, err),
            ClientError::InvalidInput(_) => {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, err)
            }
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        use crate::AppError::{Client, Server};

        match self {
            Server(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Client(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl<E> From<actix_web::error::BlockingError<E>> for AppError
where
    E: std::fmt::Debug,
    E: Into<AppError>,
{
    fn from(error: actix_web::error::BlockingError<E>) -> AppError {
        match error {
            actix_web::error::BlockingError::Error(e) => e.into(),
            actix_web::error::BlockingError::Canceled => AppError::Server(ServerError::OtherError(
                anyhow!("server thread pool is gone"),
            )),
        }
    }
}
