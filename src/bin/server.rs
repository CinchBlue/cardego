extern crate actix_web;
extern crate cardego_server;
extern crate anyhow;
extern crate thiserror;
extern crate derive_more;
extern crate actix_files;

use cardego_server::{CardDatabase};
use cardego_server::errors::{Result, ServerError, ClientError, AppError};

use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use log::{info, debug};

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

async fn route_get_card_image_as_html(
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

async fn route_get_card_image_css()
    -> Result<HttpResponse> {
    let file = std::fs::read("static/templates/card.css")?;
    Ok(
        HttpResponse::Ok()
                .content_type("text/css; charset=UTF-8")
                .body(file)
    )
}

async fn route_get_card_image_by_html(
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


async fn route_get_deck(
    path: web::Path<String>)
    -> Result<HttpResponse> {
    
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let cards = state.db.get_cards_by_deck_name(path.to_string())
            .or(Err(ClientError::ResourceNotFound))?;
    
    Ok(HttpResponse::Ok().json(cards))
}

async fn route_put_deck(
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
    let db = init_state()?;
    let mut state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
    
    let new_deck = state.db.put_deck(path.to_string(), card_ids)?;
    
    Ok(HttpResponse::Ok().json(new_deck))
}

async fn route_get_deck_cardsheet(
    path: web::Path<String>)
    -> Result<HttpResponse> {
    use cardego_server::image;
   
    // Get a connection to the database
    let db = init_state()?;
    let state = db.lock().or(Err(ServerError::DatabaseConnectionError))?;
   
    // Get the cards.
    let cards = state.db.get_cards_by_deck_name(path.to_string())
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
    
    std::fs::copy("static/templates/card.css",
        "runtime/data/cards/images/templates/card.css")?;
    info!("Copied 'card.css' from into runtime/data/cards/images/templates directory.");
    std::fs::copy("static/templates/card.css",
        "runtime/data/decks/images/templates/card.css")?;
    info!("Copied 'card.css' from into runtime/data/decks/images/templates directory.");
    
    info!("Finished reading server configuration");
    Ok(())
}

fn init_state() -> anyhow::Result<Arc<Mutex<ServerState>>> {
    debug!("Initializing database connection");
    let db = CardDatabase::new("runtime/data/databases/cards.db")
            .or(Err(ServerError::DatabaseConnectionError))?;
            
    Ok(Arc::new(Mutex::new(
        ServerState {
            db,
        }
    )))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();
   
    // Initialize all server + dependency config
    init_config()
            .or(Err(ServerError::ConfigurationError))?;
   
    // Create the HTTP server with routing below and initialize it.
    info!("Initializing server framework");
    let result = HttpServer::new(|| {
        App::new()
                .wrap(middleware::DefaultHeaders::new()
                        .header("X-API-Version", "alpha-5"))
                // ALWAYS have compression on! This is a major performance
                // boost for amount of bytes per image get!
                .wrap(middleware::Compress::default())
        .route("/", web::get().to(index))
                .service(web::scope("/cards")
                        .route("/{id}", web::get().to(route_get_card))
                        .route("/{id}/image.png",
                            web::get().to(route_get_card_image_by_html))
                        .route("/{id}/image.html",
                            web::get().to(route_get_card_image_as_html))
                        .route("/{id}/card.css",
                            web::get().to(route_get_card_image_css)))
        .service(web::scope("/decks")
                        .route("/{name}", web::get().to(route_get_deck))
                        .route("/{name}", web::put().to(route_put_deck))
                        .route("/{name}/image.png",
                            web::get().to (route_get_deck_cardsheet)))
                .service(web::scope("/search")
                    .route("/decks/{name}", web::get().to(route_query_decks))
                    .route("/cards/{name}", web::get().to(route_query_cards)))
    })
            // Local testing? Use localhost:80 for HTTP
            .bind(&args[1])?
            .run()
            .await;
    
    info!("Ending main process");
    result?;
    Ok(())
}
