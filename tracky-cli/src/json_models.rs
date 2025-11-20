use serde::Serialize;

#[derive(Serialize)]
pub struct ScanJson {
    pub files: Vec<FileInfo>,
}

#[derive(Serialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified: String,
}

#[derive(Serialize)]
pub struct ValidateJson {
    pub results: Vec<ValidationEntry>,
}

#[derive(Serialize)]
pub struct ValidationEntry {
    pub file: String,
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct BuildDbJson {
    pub files_processed: usize,
    pub errors: usize,
    pub db_path: String,
}
