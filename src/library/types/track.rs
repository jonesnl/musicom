use crate::schema::tracks;

use super::LibraryPath;

/// Track is used as the target data structure for a database query.
#[derive(Queryable)]
pub struct Track {
    pub id: i32,
    #[column_name = "path_"]
    pub path: LibraryPath,
    pub name: String,
    pub artist: Option<String>,
}

/// TrackNoId is used to insert a track into the database, having it's ID be auto-selected for you.
#[derive(Insertable)]
#[table_name="tracks"]
pub struct TrackNoId {
    #[column_name = "path_"]
    pub path: LibraryPath,
    pub name: String,
    pub artist: Option<String>,
}