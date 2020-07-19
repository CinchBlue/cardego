extern crate anyhow;
extern crate thiserror;
extern crate derive_more;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Server configuration is invalid")]
    ConfigurationError,
    #[error("Could not conenct to database")]
    DatabaseConnectionError,
    #[error("An image error occured with Cairo: {0}")]
    CairoError(cairo::BorrowError),
    #[error("File IO error occured with file: {0}")]
    FileIOError(String)
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Requested resource not found")]
    ResourceNotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Server error: `{0}`")]
    Server(ServerError),
    #[error("Client error: `{0}`")]
    Client(ClientError),
}



impl std::convert::From<AppError> for std::io::Error {
    fn from(err: AppError) -> Self {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            err
        )
    }
}



impl std::convert::From<ServerError> for crate::AppError {
    fn from(err: ServerError) -> Self {
        AppError::Server(err)
    }
}

impl std::convert::From<ServerError> for std::io::Error {
    fn from(err: ServerError) -> Self {
        std::io::Error::from(AppError::from(err))
    }
}


impl std::convert::From<ClientError> for crate::AppError {
    fn from(err: ClientError) -> Self {
        AppError::Client(err)
    }
}

impl std::convert::From<ClientError> for std::io::Error {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::ResourceNotFound => std::io::Error::new(
                std::io::ErrorKind::NotFound,
                err),
        }
    }
}
