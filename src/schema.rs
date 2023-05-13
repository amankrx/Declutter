// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        date_of_birth -> Text,
        created_at -> Text,
    }
}
