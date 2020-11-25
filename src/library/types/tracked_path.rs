use crate::schema::tracked_paths;

use super::LibraryPath;

#[derive(Queryable, Clone, PartialEq, Debug)]
pub struct TrackedPath {
    pub id: i32,
    #[column_name = "path_"]
    pub path: LibraryPath,
}

#[derive(Insertable, Clone, PartialEq, Debug)]
#[table_name="tracked_paths"]
pub struct TrackedPathNoId {
    #[column_name = "path_"]
    pub path: LibraryPath,
}

impl From<TrackedPath> for TrackedPathNoId {
    fn from(t: TrackedPath) -> Self {
        TrackedPathNoId {
            path: t.path,
        }
    }
}
