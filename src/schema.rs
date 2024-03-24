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
        lyricist -> Nullable<Text>,
        piece -> Nullable<Text>,
        opus -> Nullable<Text>,
    }
}
