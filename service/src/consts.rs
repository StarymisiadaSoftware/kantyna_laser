pub const HOOK_URL_ENVVAR: &'static str = "KANTYNA_LASER_URL";
pub const HOOK_DIR_ENVVAR: &'static str = "KANTYNA_LASER_HOOK_DIR";
pub const YTDLP_COMMAND: &'static str = "yt-dlp";
/// credit: https://stackoverflow.com/questions/19377262/regex-for-youtube-url
pub const YOUTUBE_URL_REGEX: &'static str =
    r#"^(http(s)??://)?(www\.)?((youtube\.com/watch\?v=)|(youtu.be/))([a-zA-Z0-9\-_])+"#;
/// 25 minutes
pub const LENGTH_LIMIT_SECONDS: u16 = 1500;
