// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Nullable<Integer>,
        file_type -> Text,
        path -> Text,
        score_id -> Integer,
    }
}

diesel::table! {
    scores (id) {
        id -> Nullable<Integer>,
        dedication -> Nullable<Text>,
        title -> Nullable<Text>,
        subtitle -> Nullable<Text>,
        subsubtitle -> Nullable<Text>,
        instrument -> Nullable<Text>,
        poet -> Nullable<Text>,
        composer -> Nullable<Text>,
        meter -> Nullable<Text>,
        arranger -> Nullable<Text>,
        tagline -> Nullable<Text>,
        copyright -> Nullable<Text>,
        piece -> Nullable<Text>,
        opus -> Nullable<Text>,
        ly_file_path -> Text,
        ily_file -> Nullable<Binary>,
    }
}

diesel::joinable!(files -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(files, scores,);
