pub mod db;
mod track;
mod tracked_path;

use std::path::PathBuf;

pub use track::Track;
pub use tracked_path::TrackedPath;

/// Fast refresh of the library database
///
/// This function can detect new files, but if a file already exists in the database, it doesn't
/// re-inspect its tags, it just assumes they are correct. This speeds up the library refresh
/// considerably.
pub fn fast_refresh_library() {
    let mut tracked_paths: Vec<PathBuf> = TrackedPath::iter()
        .map(|tp| tp.path.to_path_buf())
        .collect();

    if tracked_paths.is_empty() {
        return;
    }

    let mut conn = db::get_library_db().unwrap();
    let transaction = conn.transaction().unwrap();

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
            if let Some(mut track) = Track::from_path_with_conn(&path, &transaction) {
                // If the track already exists in the database, it will already hae an ID
                // associated with it. Only save the track if it isn't in the database yet.
                if track.id.is_none() {
                    track.save_with_conn(&transaction);
                }
            }
        }
    }
    transaction.commit().unwrap();
}
