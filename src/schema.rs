// @generated automatically by Diesel CLI.

diesel::table! {
    included_files (id) {
        id -> Nullable<Integer>,
        path -> Text,
    }
}

diesel::table! {
    score_included_files (score_file, included_file) {
        score_file -> Binary,
        included_file -> Nullable<Binary>,
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
    }
}

diesel::joinable!(score_included_files -> included_files (included_file));
diesel::joinable!(score_included_files -> scores (score_file));

diesel::allow_tables_to_appear_in_same_query!(
    included_files,
    score_included_files,
    scores,
);
