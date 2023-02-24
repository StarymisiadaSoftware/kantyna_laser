use crate::consts::*;
use crate::MyError;
use lazy_static::lazy_static;
use regex::Regex;

pub fn anyhow_error_to_stdio_error(err: anyhow::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

lazy_static! {
    pub static ref youtube_regex: Regex = { Regex::new(YOUTUBE_URL_REGEX).unwrap() };
}

// Only allow youtube urls
pub fn sanitize(youtube_url: String) -> Result<String, MyError> {
    let sanitized = youtube_url;
    if !youtube_regex.is_match(&sanitized) {
        Err(MyError::SanitizationError(
            "URL doesn't match any known YouTube URL".to_string(),
        ))
    } else {
        Ok(sanitized)
    }
}
