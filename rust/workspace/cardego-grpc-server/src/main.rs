#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() {
    use cardego_grpc_server::app;

    app::start_server().await.unwrap();
}

#[cfg(target_family = "wasm")]
fn main() {}
