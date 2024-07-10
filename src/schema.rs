// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
