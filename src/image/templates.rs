extern crate askama;

use askama::Template;
use crate::models::Card;

#[derive(Debug, Default, Clone)]
#[derive(Template)]
#[template(path = "single-card.html")]
pub struct SingleCardTemplate {
    pub id: i32,
    pub cardclass: String,
    pub cardclass_long: String,
    pub action: String,
    pub speed: String,
    pub name: String,
    pub desc: String,
    pub image_url: String,
}

impl SingleCardTemplate {
    pub fn new(card: &Card) -> SingleCardTemplate {
        use crate::models::TRAIT_SHORT_TO_FULLNAME;
    
        SingleCardTemplate {
            id: card.id,
            cardclass: (&card.cardclass).to_string(),
            cardclass_long: TRAIT_SHORT_TO_FULLNAME.get(&card.cardclass)
                    .unwrap().to_string(),
            action: card.action.clone(),
            speed: card.speed.clone(),
            name: card.name.clone(),
            desc: card.desc.clone(),
            image_url: card.image_url.as_ref().unwrap_or(&"".to_string())
                    .clone()
        }
    }
}

#[derive(Debug, Default)]
#[derive(Template)]
#[template(path = "cardsheet.html", escape = "none")]
pub struct CardsheetTemplate {
    pub cards: Vec<SingleCardTemplate>,
}
