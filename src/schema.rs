table! {
    cards (id) {
        id -> Integer,
        cardtype -> Text,
        name -> Text,
        cost -> Text,
        desc -> Text,
    }
}

table! {
    user_sets (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    user_sets_to_cards (id) {
        id -> Integer,
        user_set_id -> Integer,
        card_id -> Integer,
    }
}