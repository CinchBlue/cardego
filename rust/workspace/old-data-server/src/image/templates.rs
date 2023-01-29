extern crate askama;

use crate::models::Card;
use askama::Template;

#[derive(Debug, Default, Clone, Template)]
#[template(path = "../../static/templates/single-card.html")]
pub struct SingleCardTemplate {
    pub id: i32,
    pub cardclass: String,
    pub cardclass_long: String,
    pub initiative: i32,
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
            cardclass_long: TRAIT_SHORT_TO_FULLNAME
                .get(&card.cardclass)
                .unwrap()
                .to_string(),
            initiative: card.initiative,
            action: card.action.clone(),
            speed: card.speed.clone(),
            name: card.name.clone(),
            desc: card.desc.clone(),
            image_url: card.image_url.as_ref().unwrap_or(&"".to_string()).clone(),
        }
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "cardsheet.html", escape = "none")]
pub struct CardsheetTemplate {
    pub cards: Vec<SingleCardTemplate>,
}
