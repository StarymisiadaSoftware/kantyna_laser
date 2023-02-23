
#[derive(Debug)]
pub struct Song {
    url: String,
    /// in seconds
    duration: Option<u16>,
    title: Option<String>,
    miniature_url: Option<String>,
}