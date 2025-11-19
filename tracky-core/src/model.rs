use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub tracks: Vec<Track>,
}
