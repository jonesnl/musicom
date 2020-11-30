use std::path::PathBuf;

use rusqlite::{types::FromSql, Row};

/// Track is used as the target data structure for a database query.
#[derive(Clone, PartialEq, Debug)]
pub struct Track {
    pub id: i32,
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_num: Option<i32>,
}

/// TrackNoId is used to insert a track into the database, having it's ID be auto-selected for you.
#[derive(Clone, PartialEq, Debug)]
pub struct TrackNoId {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_num: Option<i32>,
}

impl From<Track> for TrackNoId {
    fn from(t: Track) -> Self {
        TrackNoId {
            path: t.path,
            title: t.title,
            artist: t.artist,
            album: t.album,
            track_num: t.track_num,
        }
    }
}

impl Track {
    pub fn from_db_row(row: &Row) -> rusqlite::Result<Self> {
        // R
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

impl TrackNoId {
    pub fn new_from_path<PB>(pb: PB) -> Option<Self>
    where
        PB: Into<PathBuf>,
    {
        let path = pb.into();
        let taglib_file = taglib::File::new(&path).ok()?;
        let tags = taglib_file.tag().ok()?;

        Some(Self {
            path: path.into(),
            title: tags.title(),
            artist: tags.artist(),
            album: tags.album(),
            track_num: tags.track().map(|val| val as i32),
        })
    }
}
