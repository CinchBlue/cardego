use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Extension, routing::get, Router};
use cardego_database::APIConnection;
use tokio::sync::Mutex;
use tracing::debug;

struct AppState {
    api_conn: APIConnection,
}

impl AppState {
    async fn auto_build() -> anyhow::Result<Self> {
        // Init the server state.
        let api_conn = APIConnection::connect().await?;
        Ok(AppState { api_conn })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt::init();

    let app_state = AppState::auto_build().await?;

    // Build our application with shared state for the connection pool.
    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(Arc::new(Mutex::new(app_state))));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "meow wow"
}

// I want to have a connection pool
// I want to make server
