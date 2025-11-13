// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        event_type -> Text,
        name -> Text,
    }
}
