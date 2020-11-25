use std::collections::VecDeque;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::util::get_database_path;

use super::types::{Track, TrackNoId};
use crate::schema::tracks;

// Embed the migrations defined at the root of the crate here.
diesel_migrations::embed_migrations!();

/// Public library methods
pub struct Library {
    db: SqliteConnection,
}

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

        Self {
            db,
        }
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

        TrackIter {
            tracks
        }
    }

    pub fn get_track(&self, id: i32) -> Option<Track> {
        tracks::table.find(id)
            .first(&self.db)
            .ok()
    }
    
    pub fn get_track_count(&self) -> usize {
        tracks::table.count()
            .first::<i64>(&self.db)
            .unwrap()
            as usize
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
        assert_eq!(lib.get_track_count(), TEST_TRACK_LIST.len(), "Incorrect library track count");

        for (track1, track2) in lib.iter_tracks().zip(TEST_TRACK_LIST.iter()) {
            let track1noid: TrackNoId = track1.into();
            assert_eq!(track1noid, *track2, "Tracks aren't equal");
        }
    }
}
