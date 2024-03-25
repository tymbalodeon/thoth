// @generated automatically by Diesel CLI.

diesel::table! {
    file_links (id) {
        id -> Integer,
        score_id -> Integer,
        included_file_id -> Integer,
    }
}

diesel::table! {
    included_files (id) {
        id -> Integer,
        path -> Text,
    }
}

diesel::table! {
    scores (id) {
        id -> Integer,
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
    }
}

diesel::joinable!(file_links -> included_files (included_file_id));
diesel::joinable!(file_links -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_links,
    included_files,
    scores,
);
