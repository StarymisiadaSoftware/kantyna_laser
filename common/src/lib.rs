use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
/// Represents the message sent from the website UI
#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequest {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub url: String,
    /// in seconds
    pub duration: Option<u16>,
    pub title: Option<String>,
    pub miniature_url: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MusicQueue {
    pub queue: VecDeque<Song>,
    pub currently_played: Option<Song>,
}