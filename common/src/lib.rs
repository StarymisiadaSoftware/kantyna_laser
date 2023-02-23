use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
/// Represents the message sent from the website UI
#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequestReply {
    /// Not empty upon error
    pub error_message: Option<String>,
    pub pos_in_queue: Option<u32>,
    /// in seconds
    pub time_to_wait: Option<u32>,
    pub song_info: Option<Song>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub url: String,
    /// in seconds
    pub duration: Option<u16>,
    pub title: Option<String>,
    pub miniature_url: Option<String>,
}

impl Song {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            duration: None,
            title: None,
            miniature_url: None,
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MusicQueuePreview {
    pub queue: MusicQueue,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MusicQueue {
    pub queue: VecDeque<Song>,
    pub currently_played: Option<Song>,
}
