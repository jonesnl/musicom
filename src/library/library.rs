use std::collections::VecDeque;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::{named_params, NO_PARAMS};

use crate::util::get_database_path;

use super::types::{Track, TrackNoId, TrackedPath};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

/// Public library methods
pub struct Library {
    db: Connection,
}

impl Library {
    pub fn new() -> Self {
        let mut db;
        if cfg!(test) {
            db = Connection::open_in_memory().unwrap();
        } else {
            let path = get_database_path().to_str().unwrap().to_string();
            db = Connection::open(path).unwrap();
        }

        self::embedded::migrations::runner().run(&mut db).unwrap();

        Self { db }
    }

    pub fn add_track(
        &self,
        track: TrackNoId,
    ) -> Result<(), ()> {
        let sql = "\
            INSERT INTO tracks (path_, title, artist, album, track_num)
                VALUES (:path, :title, :artist, :album, :track_num)";
        self.db
            .execute_named(
                sql,
                named_params! {
                    ":path": track.path.to_str().unwrap(),
                    ":title": track.title,
                    ":artist": track.artist,
                    ":album": track.album,
                    ":track_num": track.track_num,
                },
            )
            .unwrap_or_else(|e| {
                log::warn!("Could not add track to database: {}", e);
                0
            });
        Ok(())
    }

    pub fn iter_tracks(&self) -> TrackIter {
        let mut statement = self.db.prepare("SELECT * FROM tracks").unwrap();

        let tracks: Result<VecDeque<Track>, _> = statement
            .query_map(NO_PARAMS, |row| Track::from_db_row(&row))
            .unwrap()
            .collect();

        TrackIter {
            tracks: tracks.unwrap(),
        }
    }

    pub fn get_track(&self, id: i32) -> Option<Track> {
        // Given that we're exposing the database columns here, should all this be moved into the
        // tracks.rs file?
        let mut statement = self
            .db
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

    pub fn get_track_count(&self) -> usize {
        let mut statement = self.db.prepare("SELECT COUNT(*) FROM tracks").unwrap();

        statement
            .query_row(NO_PARAMS, |row| row.get::<_, u32>(0))
            .unwrap() as usize
    }

    pub fn refresh_library(&self) {
        let mut tracked_paths: Vec<PathBuf> = self
            .iter_tracked_paths()
            .map(|tp| tp.path.to_path_buf())
            .collect();

        if tracked_paths.is_empty() {
            return;
        }

        self.db.execute("BEGIN TRANSACTION", NO_PARAMS).unwrap();

        loop {
            let path = match tracked_paths.pop() {
                Some(path) => path,
                None => break,
            };

            if path.is_dir() {
                for item in path.read_dir().unwrap() {
                    tracked_paths.push(item.unwrap().path());
                }
            } else if crate::player::is_audio_file_guess(&path) {
                if let Some(track) = TrackNoId::new_from_path(&path) {
                    self.add_track(track).unwrap();
                }
            }
        }

        self.db.execute("COMMIT", NO_PARAMS).unwrap();
    }

    pub fn add_tracked_path<PB>(&self, pb: PB)
    where
        PB: Into<PathBuf>,
    {
        // Get PathBuf
        let pb = pb.into();

        self.db
            .execute_named(
                "INSERT INTO tracked_paths (path_)
                VALUES (:path)",
                named_params! {":path": pb.to_str()},
            )
            .unwrap();
    }

    pub fn iter_tracked_paths(&self) -> TrackedPathIter {
        let mut statement = self.db.prepare("SELECT * FROM tracked_paths").unwrap();

        let tracked_paths: Result<VecDeque<TrackedPath>, _> = statement
            .query_map(NO_PARAMS, |row| TrackedPath::from_db_row(row))
            .unwrap()
            .collect();

        TrackedPathIter {
            tracked_paths: tracked_paths.unwrap(),
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

pub struct TrackedPathIter {
    tracked_paths: VecDeque<TrackedPath>,
}

impl Iterator for TrackedPathIter {
    type Item = TrackedPath;

    fn next(&mut self) -> Option<Self::Item> {
        self.tracked_paths.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_TRACK_LIST: [TrackNoId; 2] = [
            TrackNoId {
                path: PathBuf::from("/tmp/test1.mp3"),
                title: Some("Test 1: The Intro".to_string()),
                artist: Some("George".to_string()),
                album: None,
                track_num: None,
            },
            TrackNoId {
                path: PathBuf::from("/tmp/test1.mp3"),
                title: Some("Test 1: The Intro".to_string()),
                artist: Some("George".to_string()),
                album: None,
                track_num: None,
            },
        ];
    }

    #[test]
    fn library_track_iter_test() {
        let lib = Library::new();
        for track in TEST_TRACK_LIST.iter() {
            lib.add_track(track.clone()).expect("Couldn't add track");
        }
        assert_eq!(
            lib.get_track_count(),
            TEST_TRACK_LIST.len(),
            "Incorrect library track count"
        );

        for (track1, track2) in lib.iter_tracks().zip(TEST_TRACK_LIST.iter()) {
            let track1noid: TrackNoId = track1.into();
            assert_eq!(track1noid, *track2, "Tracks aren't equal");
        }
    }

    #[test]
    fn library_tracked_paths() {
        let lib = Library::new();
        let tracked_paths: [PathBuf; 2] = ["/tmp/test1".into(), "/tmp/test2".into()];

        for path in tracked_paths.iter() {
            lib.add_tracked_path(path);
        }

        for (tracked_path, path) in lib.iter_tracked_paths().zip(tracked_paths.iter()) {
            assert_eq!(&tracked_path.path, path, "Paths not in database");
        }
    }
}
