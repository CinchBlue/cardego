#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::result::{Result};
use std::error::{Error};
use log::{info};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn init_config() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("runtime-config/log4rs/log4rs.yml", Default::default())?;
    info!("Initialized log4rs");
    info!("Finished reading server configuration");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init_config()?;
    
    info!("Ending main process");
    Ok(())
}
