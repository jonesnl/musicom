use std::collections::VecDeque;
use std::path::PathBuf;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::util::get_database_path;

use super::types::{Track, TrackNoId, TrackedPath, TrackedPathNoId};
use crate::schema::{tracked_paths, tracks};

// Embed the migrations defined at the root of the crate here.
diesel_migrations::embed_migrations!();

/// Public library methods
pub struct Library {
    db: SqliteConnection,
}

#[allow(dead_code)]
impl Library {
    pub fn new() -> Self {
        let path;
        if cfg!(test) {
            path = ":memory:".to_string();
        } else {
            path = get_database_path().to_str().unwrap().to_string();
        }

        let db = SqliteConnection::establish(&path).expect("Couldn't open database");

        embedded_migrations::run(&db).expect("Could not ensure database schema is correct");

        Self { db }
    }

    pub fn add_track(&self, track: TrackNoId) -> Result<(), ()> {
        diesel::insert_into(tracks::table)
            .values(track)
            .execute(&self.db)
            .unwrap_or_else(|e| {
                log::warn!("Could not add track to database: {}", e);
                0
            });
        Ok(())
    }

    pub fn iter_tracks(&self) -> TrackIter {
        let tracks: VecDeque<Track> = tracks::table.load(&self.db).unwrap().into();

        TrackIter { tracks }
    }

    pub fn get_track(&self, id: i32) -> Option<Track> {
        tracks::table.find(id).first(&self.db).ok()
    }

    pub fn get_track_count(&self) -> usize {
        tracks::table.count().first::<i64>(&self.db).unwrap() as usize
    }

    pub fn add_tracked_path<PB>(&self, pb: PB) -> Result<(), ()>
    where
        PB: Into<PathBuf>,
    {
        // Get PathBuf
        let pb = pb.into();

        let tp = TrackedPathNoId { path: pb.into() };

        tp.insert_into(tracked_paths::table)
            .execute(&self.db)
            .unwrap_or_else(|e| {
                log::warn!("Could not track directory: {}", e);
                0
            });

        Ok(())
    }

    pub fn iter_tracked_paths(&self) -> TrackedPathIter {
        let tracked_paths: VecDeque<TrackedPath> =
            tracked_paths::table.load(&self.db).unwrap().into();

        TrackedPathIter { tracked_paths }
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

    use super::super::types::LibraryPath;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_TRACK_LIST: [TrackNoId; 2] = [
            TrackNoId {
                path: LibraryPath::from("/tmp/test1.mp3"),
                name: "Test 1: The Intro".to_string(),
                artist: Some("George".to_string()),
            },
            TrackNoId {
                path: LibraryPath::from("/tmp/test1.mp3"),
                name: "Test 1: The Intro".to_string(),
                artist: Some("George".to_string()),
            },
        ];
    }

    #[test]
    fn basic_track_library_test() {
        let lib = Library::new();
        let before_cnt = lib.get_track_count();
        let track = TrackNoId {
            path: LibraryPath(PathBuf::from("/tmp/test.mp3")),
            name: "Hi".to_string(),
            artist: None,
        };
        lib.add_track(track).unwrap();

        assert_eq!(before_cnt + 1, lib.get_track_count(), "Track not added");
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
            lib.add_tracked_path(path).unwrap();
        }

        for (tracked_path, path) in lib.iter_tracked_paths().zip(tracked_paths.iter()) {
            assert_eq!(tracked_path.path, path.into(), "Paths not in database");
        }
    }
}
