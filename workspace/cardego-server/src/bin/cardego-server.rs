use std::net::SocketAddr;

use cardego_server::app::build_app;

use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt::init();

    // Build our application with shared state for the connection pool.
    let app = build_app().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// I want to have a connection pool
// I want to make server
