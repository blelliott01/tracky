use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct ScannedFile {
    pub path: String,
    pub size: u64,
    pub modified: u64,
}

pub fn scan_folder(path: &Path) -> Vec<ScannedFile> {
    let mut results = Vec::new();
    scan_recursive(path, &mut results);
    results
}

fn scan_recursive(path: &Path, out: &mut Vec<ScannedFile>) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.is_dir() {
            scan_recursive(&path, out);
        } else {
            let modified = meta
                .modified()
                .ok()
                .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            out.push(ScannedFile {
                path: path.to_string_lossy().to_string(),
                size: meta.len(),
                modified,
            });
        }
    }
}
