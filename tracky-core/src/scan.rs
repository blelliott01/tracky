use crate::{ScanResult, Track};
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_media(root: &Path) -> ScanResult {
    let mut tracks = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if matches!(ext, "m4a" | "aac" | "mp3" | "flac") {
                    let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                    tracks.push(Track {
                        path: path.to_string_lossy().into_owned(),
                        size,
                    });
                }
            }
        }
    }

    ScanResult { tracks }
}
