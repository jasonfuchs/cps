// @generated automatically by Diesel CLI.

diesel::table! {
    temperatures (created_at) {
        created_at -> Timestamp,
        temperature -> Float,
    }
}
