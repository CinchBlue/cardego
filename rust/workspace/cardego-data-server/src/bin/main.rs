use axum::{http::StatusCode, response::IntoResponse, routing::get};
use tracing::log;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Create the server w/ routes.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello_world));

    tracing::info!("meow");

    // Run our application as a hyper server.
    axum::Server::bind(&"0.0.0.0:80".parse().expect("Could not bind to IP/port"))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Could not register or use shutdown signal handler");
    println!("signal shutdown");
}

#[tracing::instrument]
async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    tracing::debug!(uri = %uri, "Fallback");
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}

#[tracing::instrument]
async fn hello_world() -> impl IntoResponse {
    log::info!("Hello world");
    (StatusCode::OK, "Hello world")
}
