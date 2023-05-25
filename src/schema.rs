// @generated automatically by Diesel CLI.

diesel::table! {
    view_tokens (id) {
        id -> Int4,
        token -> Varchar,
        filepath -> Varchar,
        expiry_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}
