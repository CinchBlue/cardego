use tonic::{transport::Server, Request, Response, Status};

use cardego_grpc::generated::cardego_data::cardego_data_service_server::{CardegoDataService, CardegoDataServiceServer};

// pub mod hello_world {
//     tonic::include_proto!("cardego_data");
// }

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl CardegoDataService for MyServer {
    async fn health_check(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        Ok(Response::new(()))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:8080".parse().unwrap();

    let server = MyServer::default();
    let server = CardegoDataServiceServer::new(server);

    let reflection_server = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(cardego_grpc::FILE_DESCRIPTOR_SET)
    .build()?;

    println!("CardegoDataServiceServer listening on {}", addr);

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(server))
        .add_service(tonic_web::enable(reflection_server))
        .serve(addr)
        .await?;

    Ok(())
}