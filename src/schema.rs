// @generated automatically by Diesel CLI.

diesel::table! {
    scores (id) {
        id -> Nullable<Integer>,
        title -> Text,
        composer -> Nullable<Text>,
    }
}
