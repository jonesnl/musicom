use std::path::{Path, PathBuf};

use url::Url;

pub fn is_audio_file_guess(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    let guess = mime_guess::from_path(path);

    let mime = if let Some(mime) = guess.first() {
        mime
    } else {
        return false;
    };

    mime.type_() == "audio"
}

pub fn create_gst_uri(path: &Path) -> Option<String> {
    // The resulting URI must be an aboslute path, so canonicalize before converting to a URI
    let canonical_path: PathBuf = path.canonicalize().ok()?;
    let uri_str = Url::from_file_path(canonical_path).ok()?.into_string();

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
        let mp3_path = TOP_DIR.join("resources/test.mp3");
        assert_eq!(is_audio_file_guess(&mp3_path), true);
    }

    #[test]
    fn test_ogg_discovery() {
        let ogg_path = TOP_DIR.join("resources/test.ogg");
        assert_eq!(is_audio_file_guess(&ogg_path), true);
    }

    #[test]
    fn test_webm_discovery() {
        let webm_path = TOP_DIR.join("resources/test.webm");
        assert_eq!(is_audio_file_guess(&webm_path), false);
    }

    #[test]
    fn test_nonexistent_discovery() {
        let nonexistent_path = TOP_DIR.join("resources/BLARG_I_DONT_EXIST.mp3");
        assert_eq!(is_audio_file_guess(&nonexistent_path), false);
    }
}
