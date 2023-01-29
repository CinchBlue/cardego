#[macro_use]
extern crate anyhow;

extern crate actix_files;
extern crate actix_web;
extern crate cardego_server;
extern crate derive_more;
extern crate thiserror;

pub mod route;

use cardego_server::errors::Result;

use actix_web::{middleware, web, App, HttpServer};
use log::info;

use cardego_server::search::create_schema;
use cardego_server::{ApplicationConfig, ServerState};
use std::sync::{Arc, Mutex};

fn init_config() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs/log4rs.yml", Default::default())?;
    info!("Finished initializing log4rs");

    std::fs::copy(
        "static/templates/card.css",
        "runtime/data/cards/images/templates/card.css",
    )?;
    info!("Copied 'card.css' from into runtime/data/cards/images/templates directory.");
    std::fs::copy(
        "static/templates/card.css",
        "runtime/data/decks/images/templates/card.css",
    )?;
    info!("Copied 'card.css' from into runtime/data/decks/images/templates directory.");

    info!("Finished reading server configuration");
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Initialize all server + dependency config
    init_config()?;

    // Create the shared application state
    let state = Arc::new(Mutex::new(ServerState {
        config: ApplicationConfig::new()?,
        schema: create_schema(),
    }));

    // Create the HTTP server with routing below and initialize it.
    info!("Initializing server framework");
    let result = HttpServer::new(move || {
        use crate::route::*;

        App::new()
            .data(state.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-API-Version", "alpha-9"))
            // ALWAYS have compression on! This is a major performance
            // boost for amount of bytes per image get!
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            .route("/cards", web::get().to(route_query_cards))
            .route("/cards", web::post().to(route_create_card))
            .service(
                web::scope("/cards")
                    .route("", web::get().to(route_query_cards))
                    .route("", web::post().to(route_create_card))
                    .route("/{id}", web::get().to(route_get_card))
                    .route("/{id}", web::put().to(route_update_card))
                    .route(
                        "/{id}/image.png",
                        web::get().to(route_get_card_image_by_html),
                    )
                    .route(
                        "/{id}/image.html",
                        web::get().to(route_get_card_image_as_html),
                    )
                    .route("/{id}/card.css", web::get().to(route_get_card_image_css)),
            )
            .service(
                web::scope("/decks")
                    .route("/{name}", web::get().to(route_get_deck))
                    .route("/{name}", web::post().to(route_create_deck))
                    .route("/{name}/image.png", web::get().to(route_get_deck_cardsheet)),
            )
            .service(
                web::scope("/search")
                    .route("/decks/{name}", web::get().to(route_query_decks))
                    .route("/cards/{name}", web::get().to(route_query_cards_by_name)),
            )
            .service(
                web::scope("/graphql")
                    .route("", web::get().to(crate::route::graphql))
                    .route("", web::post().to(crate::route::graphql)),
            )
            .route("/graphiql", web::get().to(crate::route::graphql_playground))
    })
    // Local testing? Use localhost:80 for HTTP
    .bind(&args[1])?
    .run()
    .await;

    info!("Ending main process");
    result?;
    Ok(())
}
