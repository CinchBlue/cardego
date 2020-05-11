extern crate actix_web;
extern crate cardego_server;
extern crate anyhow;
extern crate thiserror;
extern crate derive_more;


use cardego_server::{CardDatabase};
use cardego_server::errors::*;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use log::{info};

use std::sync::{Arc, Mutex};


struct ServerState {
    db: CardDatabase,
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn route_get_card(
    path: web::Path<(i32,)>)
    -> std::io::Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
   
   
    let card = state.db.get_card(path.0)
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(card))
}


async fn route_get_user_set(
    path: web::Path<String>)
    -> std::io::Result<HttpResponse> {
    
    info!("path: {}", path);
    
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let cards = state.db.get_cards_by_user_set_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

async fn route_query_user_sets(
    path: web::Path<String>)
    -> std::io::Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let user_sets = state.db.query_user_sets_by_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(user_sets))
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
                .service(web::scope("/user_set")
                        .route("/{name}", web::get().to(route_get_user_set)))
                .service(web::scope("/search")
                    .route("/user_set/{name}", web::get().to(route_query_user_sets)))
    })
            .bind("127.0.0.1:8000")?
            .run()
            .await;
    
    info!("Ending main process");
    result
}
