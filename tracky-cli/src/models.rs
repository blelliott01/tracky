use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AudioTagData {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub year: i32,
    pub track_number: i32,
    pub disc_number: i32,
    pub comment: Option<String>,
    pub genre: Option<String>,
    pub is_compilation: bool,
}
