table! {
    tracked_paths (id) {
        id -> Integer,
        path_ -> Text,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        path_ -> Text,
        title -> Nullable<Text>,
        artist -> Nullable<Text>,
        album -> Nullable<Text>,
        track_num -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    tracked_paths,
    tracks,
);
