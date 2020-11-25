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
        name -> Text,
        artist -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    tracked_paths,
    tracks,
);
