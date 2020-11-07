use std::path::{Path, PathBuf};

use gst_pbutils::Discoverer;
use gst::ClockTime;

use url::Url;

pub fn is_audio_file(path: &Path) -> bool {
    let uri_str = if let Some(val) = create_gst_uri(path) {
        val
    } else {
        return false;
    };

    // Ideally this should be less than a second for local queries, but it throws warnings if I do
    // that...
    let discoverer = Discoverer::new(ClockTime::from_seconds(1)).unwrap();
    
    let info = if let Ok(val) = discoverer.discover_uri(&uri_str) {
        val
    } else {
        return false;
    };

    if !info.get_video_streams().is_empty() {
        return false;
    }

    info.get_audio_streams().len() == 1
}

pub fn create_gst_uri(path: &Path) -> Option<String> {
    // The resulting URI must be an aboslute path, so canonicalize before converting to a URI
    let canonical_path: PathBuf = path.canonicalize().ok()?;
    let uri_str = Url::from_file_path(canonical_path)
        .ok()?
        .into_string();

    Some(uri_str)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::PathBuf;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref TOP_DIR: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    }

    #[test]
    fn test_mp3_discovery() {
        gst::init().unwrap();
        let mp3_path = TOP_DIR.join("resources/test.mp3");
        assert_eq!(is_audio_file(&mp3_path), true);
    }

    #[test]
    fn test_ogg_discovery() {
        gst::init().unwrap();
        let ogg_path = TOP_DIR.join("resources/test.ogg");
        assert_eq!(is_audio_file(&ogg_path), true);
    }

    #[test]
    fn test_webm_discovery() {
        gst::init().unwrap();
        let webm_path = TOP_DIR.join("resources/test.webm");
        assert_eq!(is_audio_file(&webm_path), false);
    }

    #[test]
    fn test_nonexistent_discovery() {
        gst::init().unwrap();
        let nonexistent_path = TOP_DIR.join("resources/BLARG_I_DONT_EXIST.mp3");
        assert_eq!(is_audio_file(&nonexistent_path), false);
    }
}
