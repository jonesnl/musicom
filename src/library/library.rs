use std::path::PathBuf;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::util::get_database_path;

use super::types::{LibraryPath, Track, TrackNoId};
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
            .expect("ERROR IN BAD PLACE");
        Ok(())
    }

    pub fn get_track(&self, id: i32) -> Option<Track> {
        tracks::table.find(id)
            .first(&self.db)
            .ok()
    }
    
    pub fn get_track_count(&self) -> i64 {
        tracks::table.count()
            .first(&self.db)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
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
}
