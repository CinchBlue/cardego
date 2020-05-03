extern crate diesel;

use super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub cardtype: String,
    pub name: String,
    pub cost: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub cardtype: &'a str,
    pub name: &'a str,
    pub cost: &'a str,
    pub desc: &'a str,
}