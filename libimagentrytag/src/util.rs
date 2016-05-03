use regex::Regex;

pub fn is_tag(s: &str) -> bool {
    Regex::new("^[a-zA-Z]([a-zA-Z0-9_-]*)$").unwrap().captures(&s[..]).is_some()
}
