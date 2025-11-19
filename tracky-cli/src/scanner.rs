use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ScannedFile {
    pub path: PathBuf,
}

pub fn scan_folder(root: &PathBuf) -> Vec<ScannedFile> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Only keep .m4a for now (same as Swift version)
        if let Some(ext) = path.extension() {
            if ext == "m4a" {
                files.push(ScannedFile {
                    path: path.to_path_buf(),
                });
            }
        }
    }

    files
}
