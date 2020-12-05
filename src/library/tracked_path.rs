use std::collections::VecDeque;
use std::path::PathBuf;

use rusqlite::{named_params, Connection, Row, NO_PARAMS};

use crate::library::db::get_library_db;

#[derive(Clone, PartialEq, Debug)]
pub struct TrackedPath {
    pub id: Option<i32>,
    pub path: PathBuf,
}

impl TrackedPath {
    pub fn from_db_row(row: &Row) -> rusqlite::Result<Self> {
        let id = row.get_unwrap(row.column_index("id")?);
        let path = row
            .get_unwrap::<_, String>(row.column_index("path_")?)
            .into();

        Ok(TrackedPath { id, path })
    }
}

// Database interactions
impl TrackedPath {
    pub fn save(&mut self) {
        let conn = get_library_db().unwrap();
        self.save_with_conn(&conn);
    }

    fn save_with_conn(&mut self, conn: &Connection) {
        conn.execute_named(
            "INSERT INTO tracked_paths (path_)
                VALUES (:path)",
            named_params! {":path": self.path.to_str()},
        )
        .unwrap();

        let new_id = conn.last_insert_rowid();

        assert_eq!(new_id as i32 as i64, new_id);
        self.id = Some(new_id as i32);
    }

    pub fn iter() -> TrackedPathIter {
        let conn = get_library_db().unwrap();
        Self::iter_with_conn(&conn)
    }

    fn iter_with_conn(conn: &Connection) -> TrackedPathIter {
        let mut statement = conn.prepare("SELECT * FROM tracked_paths").unwrap();

        let tracked_paths: Result<VecDeque<TrackedPath>, _> = statement
            .query_map(NO_PARAMS, |row| TrackedPath::from_db_row(row))
            .unwrap()
            .collect();

        TrackedPathIter {
            tracked_paths: tracked_paths.unwrap(),
        }
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
    use crate::library::db::run_migrations;

    #[test]
    fn library_tracked_paths() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        run_migrations(&mut conn);

        let mut tracked_paths = [
            TrackedPath {
                id: None,
                path: "/tmp/test1".into(),
            },
            TrackedPath {
                id: None,
                path: "/tmp/test2".into(),
            },
        ];

        for tp in tracked_paths.iter_mut() {
            tp.save_with_conn(&mut conn);
        }

        for (tracked_path, path) in TrackedPath::iter_with_conn(&mut conn).zip(tracked_paths.iter())
        {
            assert_eq!(&tracked_path, path, "Paths not in database");
        }
    }
}
