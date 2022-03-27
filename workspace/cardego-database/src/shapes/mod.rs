use serde::{Deserialize, Serialize};

use crate::models::attributes;

#[derive(Serialize, Deserialize, Debug)]
pub struct FullCard {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
    pub attributes: Vec<attributes::Model>,
}
