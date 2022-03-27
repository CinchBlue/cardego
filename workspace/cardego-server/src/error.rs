use axum::{http::StatusCode, response::Response};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("DatabaseConnectionError: Could not connect to database")]
    DatabaseConnectionError,
    #[error("FileIOError: file: {0}")]
    FileIOError(String),

    #[error(transparent)]
    DatabaseAPIError(#[from] cardego_database::error::APIError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AxumHttpError(#[from] axum::http::Error),

    #[error(transparent)]
    OtherServerError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Requested resource not found")]
    ResourceNotFound,
    #[error("Invalid input for operation found: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    OtherClientError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Server error: `{0}`")]
    Server(#[from] ServerError),
    #[error("Client error: `{0}`")]
    Client(#[from] ClientError),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> Response {
        use AppError::{Client, Server};

        (match self {
            Server(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)),
            Client(err) => (StatusCode::BAD_REQUEST, format!("{:?}", err)),
        })
        .into_response()
    }
}
