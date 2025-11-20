use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

use crate::models::AudioTagData;
use crate::tag_rules::allowed_comments;

pub fn read_tags(path: &Path) -> Option<AudioTagData> {
    let ffprobe = locate_ffprobe()?;
    let json = run_ffprobe(&ffprobe, path).ok()?;
    parse_json(&json, path).ok()
}

fn locate_ffprobe() -> Option<PathBuf> {
    let candidates = [
        "/opt/homebrew/bin/ffprobe",
        "/usr/local/bin/ffprobe",
        "/usr/bin/ffprobe",
    ];

    for c in candidates {
        if Path::new(c).exists() {
            return Some(PathBuf::from(c));
        }
    }

    None
}

fn run_ffprobe(ffprobe: &Path, file: &Path) -> Result<Value, String> {
    let tmp = std::env::temp_dir().join(format!("{}.json", Uuid::new_v4()));

    // Spawn ffprobe and dump JSON to temp file
    let output = Command::new(ffprobe)
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(file.to_string_lossy().to_string())
        .output()
        .map_err(|e| format!("ffprobe failed: {}", e))?;

    if !output.status.success() {
        return Err(format!("ffprobe exited with {}", output.status));
    }

    fs::write(&tmp, &output.stdout).map_err(|_| "Could not write ffprobe output".to_string())?;

    let data = fs::read(&tmp).map_err(|_| "Could not read ffprobe JSON".to_string())?;

    let _ = fs::remove_file(&tmp);

    let json: Value = serde_json::from_slice(&data).map_err(|_| "Invalid JSON".to_string())?;

    Ok(json)
}

fn parse_json(json: &Value, path: &Path) -> Result<AudioTagData, String> {
    let format = json.get("format").ok_or("missing format")?;
    let tags = format.get("tags").ok_or("missing tags")?;

    let s = |key: &str| {
        tags.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };

    let artist = s("artist").ok_or("missing artist")?;
    let album = s("album").ok_or("missing album")?;
    let title = s("title").ok_or("missing title")?;

    let date_str = s("date").ok_or("missing year")?;
    let year: i32 = date_str
        .get(0..4)
        .ok_or("bad year")?
        .parse()
        .map_err(|_| "bad year")?;

    let raw_comment = s("comment").unwrap_or_default().trim().to_owned();

    let comment = if raw_comment.is_empty() {
        None
    } else if allowed_comments().contains(raw_comment.as_str()) {
        Some(raw_comment)
    } else {
        None
    };

    let parse_intlike =
        |raw: Option<String>| -> Option<i32> { raw?.split('/').next()?.parse::<i32>().ok() };

    let is_singles = album == "AAA Singles";

    let track_number = if is_singles {
        0
    } else {
        parse_intlike(s("track"))
            .or_else(|| parse_intlike(s("tracknumber")))
            .or_else(|| parse_intlike(s("trkn")))
            .or_else(|| parse_intlike(s("iTunes_CDDB_TrackNumber")))
            .unwrap_or(0)
    };

    let disc_number = if is_singles {
        0
    } else {
        parse_intlike(s("disc"))
            .or_else(|| parse_intlike(s("discnumber")))
            .unwrap_or(0)
    };

    let cpil_flag =
        parse_intlike(s("compilation")) == Some(1) || parse_intlike(s("cpil")) == Some(1);

    let lower = path.to_string_lossy().to_lowercase();
    let folder_comp = lower.contains("/_ comps/") || lower.contains("/_ singles/");

    let is_compilation = cpil_flag || folder_comp;

    Ok(AudioTagData {
        artist,
        album,
        title,
        year,
        track_number,
        disc_number,
        comment,
        genre: s("genre"),
        is_compilation,
    })
}
