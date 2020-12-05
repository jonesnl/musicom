use std::collections::VecDeque;
use std::path::PathBuf;

use rusqlite::{named_params, types::FromSql, Connection, Row, NO_PARAMS};

use crate::library::db::get_library_db;

#[derive(Clone, Debug)]
pub struct Track {
    pub id: Option<i32>,
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_num: Option<i32>,
}

impl PartialEq<Track> for Track {
    fn eq(&self, other: &Track) -> bool {
        self.path == other.path
            && self.title == other.title
            && self.artist == other.artist
            && self.album == other.album
            && self.track_num == other.track_num
    }
}

impl Track {
    pub fn from_db_row(row: &Row) -> rusqlite::Result<Self> {
        fn get_column<T: FromSql>(col: &str, row: &Row) -> rusqlite::Result<T> {
            row.column_index(col).map(|idx| row.get(idx))?
        };

        let id = get_column("id", row)?;
        let path = row
            .get_unwrap::<_, String>(row.column_index("path_")?)
            .into();

        let title = get_column("title", row).ok();
        let artist = get_column("artist", row).ok();
        let album = get_column("album", row).ok();
        let track_num = get_column("track_num", row).ok();
        Ok(Track {
            id,
            path,
            title,
            artist,
            album,
            track_num,
        })
    }
}

// Functions to fetch tracks from the database
impl Track {
    #[allow(dead_code)]
    pub fn from_path<PB>(pb: PB) -> Option<Self>
    where
        PB: Into<PathBuf>,
    {
        let path = pb.into();
        let conn = get_library_db().unwrap();

        Self::from_path_with_conn(path, &conn)
    }

    pub fn from_path_with_conn<PB>(pb: PB, conn: &Connection) -> Option<Self>
    where
        PB: Into<PathBuf>,
    {
        let path = pb.into();

        let mut statement = conn
            .prepare(
                "SELECT id, path_ FROM tracks
                WHERE path_ = :path",
            )
            .unwrap();

        let id_opt: Option<i32> = statement
            .query_row_named(named_params! {":path": path.to_str() }, |row| row.get(0))
            .ok();

        if let Some(id) = id_opt {
            return Self::get_with_conn(&conn, id);
        }

        let taglib_file = taglib::File::new(&path).ok()?;
        let tags = taglib_file.tag().ok()?;

        Some(Self {
            id: None,
            path: path.into(),
            title: tags.title(),
            artist: tags.artist(),
            album: tags.album(),
            track_num: tags.track().map(|val| val as i32),
        })

    }

    #[allow(dead_code)]
    pub fn get(id: i32) -> Option<Self> {
        let conn = get_library_db().unwrap();
        Self::get_with_conn(&conn, id)
    }

    pub fn get_with_conn(conn: &Connection, id: i32) -> Option<Self> {
        // Given that we're exposing the database columns here, should all this be moved into the
        // tracks.rs file?
        let mut statement = conn
            .prepare(
                "SELECT * FROM tracks
                    WHERE id = :id
                    LIMIT 1",
            )
            .unwrap();

        statement
            .query_row_named(named_params! {":id": id}, |row| Track::from_db_row(&row))
            .ok()
    }

    #[allow(dead_code)]
    pub fn save(&mut self) {
        let conn = get_library_db().unwrap();
        self.save_with_conn(&conn)
    }

    pub fn save_with_conn(&mut self, conn: &Connection) {
        let sql = "\
            INSERT OR REPLACE INTO tracks (id, path_, title, artist, album, track_num)
                VALUES (:id, :path, :title, :artist, :album, :track_num)";
        conn.execute_named(
            sql,
            named_params! {
                ":id": self.id,
                ":path": self.path.to_str().unwrap(),
                ":title": self.title,
                ":artist": self.artist,
                ":album": self.album,
                ":track_num": self.track_num,
            },
        )
        .unwrap_or_else(|e| {
            log::warn!("Could not add track to database: {}", e);
            0
        });

        let new_id = conn.last_insert_rowid();

        assert_eq!(new_id as i32 as i64, new_id);
        self.id = Some(new_id as i32);
    }

    #[allow(dead_code)]
    pub fn get_track_count() -> usize {
        let conn = get_library_db().unwrap();
        Self::get_track_count_with_conn(&conn)
    }

    #[allow(dead_code)]
    fn get_track_count_with_conn(conn: &Connection) -> usize {
        let mut statement = conn.prepare("SELECT COUNT(*) FROM tracks").unwrap();

        statement
            .query_row(NO_PARAMS, |row| row.get::<_, u32>(0))
            .unwrap() as usize
    }

    pub fn iter() -> TrackIter {
        let conn = get_library_db().unwrap();
        Self::iter_with_conn(&conn)
    }

    pub fn iter_with_conn(conn: &Connection) -> TrackIter {
        let mut statement = conn.prepare("SELECT * FROM tracks").unwrap();

        let tracks: Result<VecDeque<Track>, _> = statement
            .query_map(NO_PARAMS, |row| Track::from_db_row(&row))
            .unwrap()
            .collect();

        TrackIter {
            tracks: tracks.unwrap(),
        }
    }
}

pub struct TrackIter {
    tracks: VecDeque<Track>,
}

impl Iterator for TrackIter {
    type Item = Track;

    fn next(&mut self) -> Option<Self::Item> {
        self.tracks.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::library::db::run_migrations;

    #[test]
    fn iter_tracks_in_library() {
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

        assert_eq!(
            Track::get_track_count_with_conn(&mut conn),
            tracks.len(),
            "Incorrect library track count"
        );

        for (track1, track2) in Track::iter_with_conn(&mut conn).zip(tracks.iter()) {
            let track1noid: Track = track1.into();
            assert_eq!(track1noid, *track2, "Tracks aren't equal");
        }
    }
}
