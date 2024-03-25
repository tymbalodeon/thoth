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
        id -> Integer,
        piece -> Nullable<Text>,
        subtitle -> Nullable<Text>,
        subsubtitle -> Nullable<Text>,
        arranger -> Nullable<Text>,
        tagline -> Nullable<Text>,
        lyricist -> Nullable<Text>,
        instrument -> Nullable<Text>,
        dedication -> Nullable<Text>,
        poet -> Nullable<Text>,
        opus -> Nullable<Text>,
        meter -> Nullable<Text>,
        copyright -> Nullable<Text>,
        title -> Nullable<Text>,
        composer -> Nullable<Text>,
    }
}

diesel::joinable!(files -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(files, scores,);
