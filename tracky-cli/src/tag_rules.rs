use std::collections::HashSet;

pub fn allowed_comments() -> HashSet<String> {
    HashSet::from(["".to_string(), "LIKED".to_string()])
}
