use std::path::PathBuf;

use rusqlite::Row;

#[derive(Clone, PartialEq, Debug)]
pub struct TrackedPath {
    pub id: i32,
    pub path: PathBuf,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TrackedPathNoId {
    pub path: PathBuf,
}

impl From<TrackedPath> for TrackedPathNoId {
    fn from(t: TrackedPath) -> Self {
        TrackedPathNoId {
            path: t.path,
        }
    }
}

impl TrackedPath {
    pub fn from_db_row(row: &Row) -> rusqlite::Result<Self> {
        let id = row.get_unwrap(row.column_index("id")?);
        let path = row
            .get_unwrap::<_, String>(row.column_index("path_")?)
            .into();

        Ok(TrackedPath {
            id,
            path,
        })
    }
}
