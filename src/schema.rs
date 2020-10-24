table! {
    cards (id) {
        id -> Integer,
        cardclass -> Text,
        action -> Text,
        speed -> Text,
        initiative -> Integer,
        name -> Text,
        desc -> Text,
        image_url -> Nullable<Text>,
    }
}

table! {
    decks (id) {
        id -> Integer,
        decktype -> Text,
        name -> Text,
    }
}

table! {
    decks_cards_relation (id) {
        id -> Integer,
        deck_id -> Integer,
        card_id -> Integer,
    }
}

table! {
    card_attributes (id) {
        id -> Integer,
        name -> Text,
        order -> Integer,
    }
}

table! {
    cards_card_attributes_relation (id) {
        id -> Integer,
        card_id -> Integer,
        card_attribute_id -> Integer,
    }
}