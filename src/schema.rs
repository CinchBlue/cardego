table! {
    cards (id) {
        id -> Integer,
        cardclass -> Text,
        action -> Text,
        speed -> Text,
        name -> Text,
        desc -> Text,
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