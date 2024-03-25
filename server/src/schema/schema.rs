// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        full_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}
