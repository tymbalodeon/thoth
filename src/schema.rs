// @generated automatically by Diesel CLI.

diesel::table! {
    included_files (id) {
        id -> Nullable<Integer>,
        path -> Text,
    }
}

diesel::table! {
    linked_files (id) {
        id -> Nullable<Integer>,
        score_id -> Integer,
        included_file_id -> Nullable<Integer>,
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

diesel::joinable!(linked_files -> included_files (included_file_id));
diesel::joinable!(linked_files -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(
    included_files,
    linked_files,
    scores,
);
