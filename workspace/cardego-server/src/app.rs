use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use cardego_database::APIConnection;
use tokio::sync::Mutex;

use crate::{
    error::{AppError, AppResult},
    routes::{index, query_cards},
};

/// The shared state between tasks for the app.
pub struct AppState {
    pub api_conn: APIConnection,
}

impl AppState {
    pub async fn auto_build() -> AppResult<Self> {
        // Init the server state.
        let api_conn = APIConnection::connect()
            .await
            .map_err(|err| AppError::Server(err.into()))?;
        Ok(AppState { api_conn })
    }
}

/// Build the app from config and setup the routes/middleware.
///
/// TODO: Change to use config struct.
pub async fn build_app() -> AppResult<Router> {
    let app_state = AppState::auto_build().await?;

    // Build our application with shared state for the connection pool.
    let app = Router::new()
        .route("/", get(index))
        .route("/cards", get(query_cards))
        .layer(Extension(Arc::new(Mutex::new(app_state))));
    Ok(app)
}
