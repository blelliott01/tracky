use crate::models::AudioTagData;
use crate::tag_rules::allowed_comments;

#[derive(Debug)]
pub enum TagValidationError {
    MissingRequired(String),
    BadSinglesRule(String),
    BadCompsRule(String),
    BadRegularRule(String),
    BadComment(String),
}

impl std::fmt::Display for TagValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagValidationError::MissingRequired(msg) => write!(f, "{}", msg),
            TagValidationError::BadSinglesRule(msg) => write!(f, "{}", msg),
            TagValidationError::BadCompsRule(msg) => write!(f, "{}", msg),
            TagValidationError::BadRegularRule(msg) => write!(f, "{}", msg),
            TagValidationError::BadComment(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TagValidationError {}

pub fn validate(tags: &AudioTagData, file: &str) -> Result<(), TagValidationError> {
    // ───────────────────── REQUIRED ─────────────────────
    if tags.artist.trim().is_empty() {
        return Err(TagValidationError::MissingRequired(format!(
            "{file}: missing ARTIST"
        )));
    }
    if tags.album.trim().is_empty() {
        return Err(TagValidationError::MissingRequired(format!(
            "{file}: missing ALBUM"
        )));
    }
    if tags.title.trim().is_empty() {
        return Err(TagValidationError::MissingRequired(format!(
            "{file}: missing TITLE"
        )));
    }
    if tags.year <= 0 {
        return Err(TagValidationError::MissingRequired(format!(
            "{file}: missing YEAR"
        )));
    }

    // Comment whitelist
    if let Some(c) = &tags.comment {
        if !allowed_comments().contains(c.as_str()) {
            return Err(TagValidationError::BadComment(format!(
                "{file}: comment must be empty or 'LIKED'"
            )));
        }
    }

    // ───────────────────── AAA SINGLES ─────────────────────
    if tags.album == "AAA Singles" {
        if tags.track_number != 0 {
            return Err(TagValidationError::BadSinglesRule(format!(
                "{file}: Singles must NOT have track number"
            )));
        }
        if tags.disc_number != 0 {
            return Err(TagValidationError::BadSinglesRule(format!(
                "{file}: Singles must NOT have disc number"
            )));
        }
        return Ok(());
    }

    // ───────────────────── COMPILATIONS ─────────────────────
    if tags.is_compilation && tags.album != "AAA Singles" {
        if tags.track_number == 0 {
            return Err(TagValidationError::BadCompsRule(format!(
                "{file}: compilation missing track number"
            )));
        }
        if tags.disc_number == 0 {
            return Err(TagValidationError::BadCompsRule(format!(
                "{file}: compilation missing disc number"
            )));
        }
        return Ok(());
    }

    // ───────────────────── REGULAR ALBUMS ─────────────────────
    if !tags.is_compilation {
        if tags.track_number == 0 {
            return Err(TagValidationError::BadRegularRule(format!(
                "{file}: regular track missing track number"
            )));
        }
        if tags.disc_number == 0 {
            return Err(TagValidationError::BadRegularRule(format!(
                "{file}: regular track missing disc number"
            )));
        }
        return Ok(());
    }

    Ok(())
}
