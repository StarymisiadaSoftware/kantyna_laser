use crate::MyError;
use regex::Regex;

pub fn anyhow_error_to_stdio_error(err: anyhow::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

// Only allow youtube urls
pub fn sanitize(youtube_url: String) -> Result<String, MyError> {
    let sanitized = youtube_url;
    // credit: https://stackoverflow.com/questions/19377262/regex-for-youtube-url
    let regexp =
        r#"^(http(s)??://)?(www\.)?((youtube\.com/watch\?v=)|(youtu.be/))([a-zA-Z0-9\-_])+"#;
    let rx = Regex::new(regexp).unwrap();
    if !rx.is_match(&sanitized) {
        Err(MyError::SanitizationError(
            "URL doesn't match any know YouTube URL".to_string(),
        ))
    } else {
        Ok(sanitized)
    }
}
