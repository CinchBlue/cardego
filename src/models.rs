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


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct UserSet {
    pub id: i32,
    pub name: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct UserSetToCard {
    pub id: i32,
    pub user_set_id: i32,
    pub card_id: i32,
}
