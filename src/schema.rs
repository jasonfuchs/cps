// @generated automatically by Diesel CLI.

diesel::table! {
    temperatures (id) {
        id -> Integer,
        temperature -> Float,
        created_at -> Timestamp,
    }
}
