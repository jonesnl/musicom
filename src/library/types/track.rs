use std::path::PathBuf;

use crate::schema::tracks;

use super::LibraryPath;

/// Track is used as the target data structure for a database query.
#[derive(Queryable, Clone, PartialEq, Debug)]
pub struct Track {
    pub id: i32,
    #[column_name = "path_"]
    pub path: LibraryPath,
    pub name: String,
    pub artist: Option<String>,
}

/// TrackNoId is used to insert a track into the database, having it's ID be auto-selected for you.
#[derive(Insertable, Clone, PartialEq, Debug)]
#[table_name="tracks"]
pub struct TrackNoId {
    #[column_name = "path_"]
    pub path: LibraryPath,
    pub name: String,
    pub artist: Option<String>,
}

impl From<Track> for TrackNoId {
    fn from(t: Track) -> Self {
        TrackNoId {
            path: t.path,
            name: t.name,
            artist: t.artist,
        }
    }
}

impl TrackNoId {
    pub fn new_from_path<PB>(pb: PB) -> Option<Self>
    where
        PB: Into<PathBuf>
    {
        let path = pb.into();
        let taglib_file = taglib::File::new(&path).ok()?;
        let tags = taglib_file.tag().ok()?;

        Some(Self {
            path: path.into(),
            name: tags.title()?,
            artist: tags.artist(),
        })
    }
}
