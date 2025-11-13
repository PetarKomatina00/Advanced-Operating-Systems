// @generated automatically by Diesel CLI.

diesel::table! {
    lekovi (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        price -> Nullable<Numeric>,
    }
}
