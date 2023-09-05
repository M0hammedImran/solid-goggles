// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        is_done -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
