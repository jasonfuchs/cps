// @generated automatically by Diesel CLI.

diesel::table! {
    temperatures (id) {
        id -> Nullable<Integer>,
        temperature -> Nullable<Float>,
        created_at -> Nullable<Timestamp>,
    }
}
