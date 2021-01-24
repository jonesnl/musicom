use rusqlite::{named_params, Connection, NO_PARAMS};

use crate::library::db::get_library_db;

use super::track::Track;

pub struct Album {
    pub album: String,
    pub track_list: Vec<Track>,
}

impl Album {
    pub fn get_album(title: &str) -> Self {
        let conn = get_library_db().unwrap();
        Self::get_album_with_conn(&conn, title)
    }

    pub fn get_album_with_conn(conn: &Connection, title: &str) -> Self {
        let mut statement = conn
            .prepare(
                "SELECT * FROM tracks
                    WHERE album = :album"
            )
            .unwrap();

        let track_list: Result<Vec<Track>, _> = statement
            .query_map_named(named_params! {":album": title },
                |row| Track::from_db_row(&row))
            .unwrap()
            .collect();

        Album {
            album: title.into(),
            track_list: track_list.unwrap(),
        }
    }

    pub fn get_all_album_keys() -> Vec<String> {
        let conn = get_library_db().unwrap();

        Self::get_all_album_keys_with_conn(&conn)
    }

    pub fn get_all_album_keys_with_conn(conn: &Connection) -> Vec<String> {
        let mut statement = conn
            .prepare(
                "SELECT DISTINCT t.album
                    FROM tracks t
                    WHERE t.album NOT NULL",
            )
            .unwrap();

        statement
            .query_map(NO_PARAMS, |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn iter_tracks(&self) -> impl Iterator<Item = &Track> {
        self.track_list.iter()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::library::db::run_migrations;

    use crate::library::Track;

    #[test]
    fn album_keys() {
        let mut tracks = [
            Track {
                id: None,
                path: PathBuf::from("/tmp/test1.mp3"),
                title: Some("Test 1: The Intro".to_string()),
                artist: Some("George".to_string()),
                album: None,
                track_num: None,
            },
            Track {
                id: None,
                path: PathBuf::from("/tmp/test2.mp3"),
                title: Some("Test 1: The Intro".to_string()),
                artist: Some("George".to_string()),
                album: None,
                track_num: None,
            },
        ];
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        run_migrations(&mut conn);

        for track in tracks.iter_mut() {
            track.save_with_conn(&mut conn);
        }
    }
}
