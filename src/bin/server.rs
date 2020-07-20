extern crate actix_web;
extern crate cardego_server;
extern crate anyhow;
extern crate thiserror;
extern crate derive_more;
extern crate cairo;

use cardego_server::{CardDatabase};
use cardego_server::errors::{Result, ServerError, ClientError};

use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use log::{info, debug, warn};

use std::sync::{Arc, Mutex};
use std::fs::{File};

struct ServerState {
    db: CardDatabase,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn route_get_card(
    path: web::Path<(i32,)>)
    -> Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
  
    let card = state.db.get_card(path.0)
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(card))
}

async fn route_get_card_image(
    path: web::Path<(i32,)>)
    -> Result<HttpResponse> {
    
    use cardego_server::image;
    
    let card_id = path.0;

    // Get the card data from the database.
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    let card_info = state.db.get_card(card_id)
            .or(Err(ClientError::ResourceNotFound))?;
    
    debug!("got card info: {:?}", &card_info);
    
    // Get the associated image file from the image url.
    let image_filename = &card_info.image_url;
    let mut local_image_filename: Option<String> = None;
    
    if let Some(image_file) = &image_filename {
        debug!("Trying to get file {:?}", image_file);
        local_image_filename = Some(
            image::retrieve_image(&image_file, card_id).await?);
        debug!("Done trying to get file {:?}", image_file);
    }
    
    if image_filename.is_none() {
        warn!("No image file for card id {:?}",
            card_id);
    }
   
    // Generate the card's raw image data.
    let surface = image::render_surface(&card_info, local_image_filename)?;
 
    // Write the raw image out to local storage.
    let out_file_name = format!("runtime/data/cards/images/{}.png", card_id);
    let mut out_file = File::create(&out_file_name)?;
    surface.write_to_png(&mut out_file)
            .or(Err(ServerError::FileIOError(out_file_name.clone())))?;

    // Read the formatted data back in to be transmitted over the wire.
    let new_file = File::open(&out_file_name)?;
    let length = new_file.metadata()?.len();
    let buffer = std::fs::read(&out_file_name)?;
    
    // We currently use PNG as our format.
    Ok(
        HttpResponse::Ok()
                .content_type("image/png")
                .content_length(length)
                .body(buffer)
    )
}


async fn route_get_deck(
    path: web::Path<String>)
    -> Result<HttpResponse> {
    
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let cards = state.db.get_cards_by_deck_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

async fn route_query_decks(
    path: web::Path<String>)
    -> Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let decks = state.db.query_decks_by_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(decks))
}

async fn route_query_cards(
    path: web::Path<String>)
    -> Result<HttpResponse> {
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let cards = state.db.query_cards_by_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

fn init_config() -> anyhow::Result<()>  {
    log4rs::init_file("config/log4rs/log4rs.yml", Default::default())?;
    info!("Finished initializing log4rs");
    info!("Finished reading server configuration");
    Ok(())
}

fn init_state() -> Result<Arc<Mutex<ServerState>>> {
    debug!("Initializing database connection");
    let db = CardDatabase::new("runtime/data/databases/cards.db")
            .or(Err(ServerError::DatabaseConnectionError))?;
    
    Ok(Arc::new(Mutex::new(ServerState { db })))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    init_config()
            .or(Err(ServerError::ConfigurationError))?;
    
    info!("Initializing server framework");
    let result = HttpServer::new(|| {
        App::new()
                .wrap(middleware::DefaultHeaders::new()
                        .header("X-API-Version", "alpha-3"))
        .route("/", web::get().to(index))
                .service(web::scope("/cards")
                        .route("/{id}", web::get().to(route_get_card))
                        .route("/{id}/image.png",
                            web::get().to(route_get_card_image)))
        .service(web::scope("/decks")
                        .route("/{name}", web::get().to(route_get_deck)))
                .service(web::scope("/search")
                    .route("/decks/{name}", web::get().to(route_query_decks))
                    .route("/cards/{name}", web::get().to(route_query_cards)))
    })
            .bind("localhost:8000")?
            //.bind("192.168.0.5:8000")?
            .run()
            .await;
    
    info!("Ending main process");
    result?;
    Ok(())
}