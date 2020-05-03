extern crate actix_web;
extern crate cardego_server;
extern crate anyhow;
extern crate thiserror;
extern crate derive_more;

use cardego_server::{CardDatabase};


use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

use anyhow::{Result, Context};

use log::{info};
use std::sync::{Arc, Mutex};
use std::error::Error;

// impl std::convert::From<Error> for std::io::Error {
//     fn from(e: anyhow::Error) -> Self {
//         unimplemented!()
//     }
// }

struct ServerState {
    db: CardDatabase,
}

#[derive(thiserror::Error, Debug)]
enum AppError {
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

#[derive(thiserror::Error, Debug)]
enum ServerError {
    #[error("Server configuration is invalid")]
    ConfigurationError,
    #[error("Could not conenct to database")]
    DatabaseConnectionError,
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

#[derive(thiserror::Error, Debug)]
enum ClientError {
    #[error("Requested resource not found")]
    ResourceNotFound,
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

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn route_get_card(
    path: web::Path<(i32,)>)
    -> std::io::Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
   
   
    let card = state.db.get_card(path.0).or(
        Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(card))
}


fn init_config() -> anyhow::Result<()>  {
    log4rs::init_file("config/log4rs/log4rs.yml", Default::default())?;
    info!("Finished init log4rs");
    info!("Finished reading server configuration");
    Ok(())
}

fn init_state() -> std::io::Result<Arc<Mutex<ServerState>>> {
    info!("Init database connection");
    let db = CardDatabase::new ("runtime/data/databases/cards.db")
            .or(Err(ServerError::DatabaseConnectionError))?;
    
    Ok(Arc::new(Mutex::new(ServerState { db })))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_config()
            .or(Err(ServerError::ConfigurationError))?;
    
    info!("Init server framework");
    let result = HttpServer::new(|| {
        App::new()
                .route("/", web::get().to(index))
                .service(web::scope("/cards")
                                .route("/{id}", web::get().to(route_get_card)))
    })
            .bind("127.0.0.1:8000")?
            .run()
            .await;
    
    info!("Ending main process");
    result
}
