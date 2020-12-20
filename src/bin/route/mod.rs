extern crate cardego_server;
extern crate actix_web;
extern crate anyhow;
extern crate thiserror;
extern crate derive_more;
extern crate actix_files;

use cardego_server::database::{DatabaseContext};
use cardego_server::{ServerState};
use cardego_server::errors::{Result, ServerError, ClientError, AppError};

use actix_web::{web, Responder, HttpResponse, HttpRequest};
use log::{info, debug};

use std::sync::{Arc, Mutex, MutexGuard};
use std::fs::{File};
use cardego_server::models::{FullCardData, NewFullCardData};
use cardego_server::search::Schema;
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub fn get_connection(
    state: &ServerState)
    -> Result<DatabaseContext> {
    DatabaseContext::new(state.config.database_endpoint.as_str())
            .or(Err(AppError::Server(ServerError::DatabaseConnectionError)))
}

pub fn lock_server_state(
    state: &web::Data<Arc<Mutex<ServerState>>>)
    -> Result<MutexGuard<ServerState>> {
   
    let result = state.lock().or(Err(AppError::Server(
        ServerError::OtherError(anyhow::anyhow!(
        "Failed to establish a lock on the server state")))))?;
    Ok(result)
}

pub async fn route_get_card(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<(i32,)>)
    -> Result<HttpResponse> {
   
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    
    let card = db.get_card(path.0)
            .or(Err(ClientError::ResourceNotFound))?;
    
    let card_attributes = db.get_card_attributes_by_card_id(path.0)
            .map(|v| Some(v))
            .unwrap_or(None);
    
    Ok(HttpResponse::Ok().json(FullCardData {
        id: card.id,
        cardclass: card.cardclass,
        action: card.action,
        speed: card.speed,
        initiative: card.initiative,
        name: card.name,
        desc: card.desc,
        image_url: card.image_url,
        card_attributes,
    }))
}

pub async fn route_get_card_image_as_html(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<(i32,)>)
    -> Result<HttpResponse> {
    
    use cardego_server::image;
    
    let card_id = path.0;
    
    // Get the card data from the database.
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    let card_info = db.get_card(card_id)
            .or(Err(ClientError::ResourceNotFound))?;
    
    debug!("got card info: {:?}", &card_info);
    
    // Generate the image from the template and write it into file.
    let out_html_string = image::generate_card_image_html_string(
        &card_info)?;
    
    info!("Generated HTML for {:?}", &card_info.id);
    
    // We currently use PNG as our format.
    Ok(
        HttpResponse::Ok()
                .content_type("text/html; charset=UTF-8")
                .body(out_html_string)
    )
}

pub async fn route_get_card_image_css()
    -> Result<HttpResponse> {
    let file = std::fs::read("static/templates/card.css")?;
    Ok(
        HttpResponse::Ok()
                .content_type("text/css; charset=UTF-8")
                .body(file)
    )
}

pub async fn route_get_card_image_by_html(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<(i32,)>)
    -> Result<HttpResponse> {
    
    use cardego_server::image;
    
    let card_id = path.0;
    
    // Get the card data from the database.
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    let card_info = db.get_card(card_id)
            .or(Err(ClientError::ResourceNotFound))?;
    
    debug!("got card info: {:?}", &card_info);
    
    // Generate the image from the template and write it into file.
    let out_file_name = image::generate_card_image(
        &card_info)?;
    
    // Read the formatted data back in to be transmitted over the wire.
    let new_file = File::open(&out_file_name)?;
    let length = new_file.metadata()?.len();
    let buffer = std::fs::read(&out_file_name)?;
    
    info!("Generated local image {:?}", out_file_name);
    
    // We currently use PNG as our format.
    Ok(
        HttpResponse::Ok()
                .content_type("image/png")
                .content_length(length)
                .body(buffer)
    )
}

pub async fn route_create_card(
    state: web::Data<Arc<Mutex<ServerState>>>,
    req: HttpRequest,
    card: web::Json<NewFullCardData>)
    -> Result<HttpResponse> {

    let state = lock_server_state(&state)?;
    let mut db = get_connection(&state)?;
    
    let full_card_data: FullCardData = db.create_card(&card)?;
   
    Ok(HttpResponse::Created()
            .header("Location", format!("{}/{}",
                req.path(), full_card_data.id))
            .finish())
}

pub async fn route_update_card(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<i32>,
    card: web::Json<FullCardData>)
    -> Result<HttpResponse> {
    
    let state = lock_server_state(&state)?;
    let mut db = get_connection(&state)?;
    
    let mut card: FullCardData = card.into_inner();
    card.id = *path;
    
    let full_card_data: FullCardData = db.update_card(card)?;
    
    Ok(HttpResponse::Ok().finish())
}


pub async fn route_get_deck(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<String>)
    -> Result<HttpResponse> {
    
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    
    let cards = db.get_cards_by_deck_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

pub async fn route_create_deck(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<String>,
    body: String)
    -> Result<HttpResponse> {
    
    // Validate that the body is a list of i32
    let strings: Vec<&str> = body.split_whitespace().collect();
    let card_ids: Vec<i32> = strings.iter()
            .flat_map(|s| s.parse())
            .collect();
    
    if strings.len() != card_ids.len() {
        return Err(AppError::Client(ClientError::InvalidInput(
            "One of the strings provided was not a valid card id".to_owned())
        ));
    }
    
    // Init the database state
    let state = lock_server_state(&state)?;
    let mut db = get_connection(&state)?;
    
    let new_deck = db.create_deck(path.to_string(), card_ids)?;
    
    Ok(HttpResponse::Ok().json(new_deck))
}

pub async fn route_get_deck_cardsheet(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<String>)
    -> Result<HttpResponse> {
    use cardego_server::image;
    
    // Get a connection to the database
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    
    // Get the cards.
    let cards = db.get_cards_by_deck_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    // Generate the image from the template and write it into file.
    let out_file_name = image::generate_deck_cardsheet_image(
        &path,
        cards)?;
    
    // Read the formatted data back in to be transmitted over the wire.
    let new_file = File::open(&out_file_name)?;
    let length = new_file.metadata()?.len();
    let buffer = std::fs::read(&out_file_name)?;
    
    info!("Generated local image {:?}", out_file_name);
    
    // We currently use PNG as our format.
    Ok(
        HttpResponse::Ok()
                .content_type("image/png")
                .content_length(length)
                .body(buffer)
    )
}

pub async fn route_query_decks(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<String>)
    -> Result<HttpResponse> {
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    
    let decks = db.query_decks_by_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(decks))
}

pub async fn route_query_cards(
    state: web::Data<Arc<Mutex<ServerState>>>,
    path: web::Path<String>)
    -> Result<HttpResponse> {
    let state = lock_server_state(&state)?;
    let db = get_connection(&state)?;
    
    let cards = db.query_cards_by_name(path.to_string()).or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

pub async fn graphql(
    state: web::Data<Arc<Mutex<ServerState>>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse> {
    
    // Handle the incoming request and return a string result or error
    let res = web::block(move || {
        let state = lock_server_state(&state)?;
        let db = get_connection(&state)?;
        let res = data.execute(&state.schema, &db);
        Ok::<_, anyhow::Error>(serde_json::to_string(&res)?)
    }).await?;
    
    Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res))
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source("/graphql")))
}
