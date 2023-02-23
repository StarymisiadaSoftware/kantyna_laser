use serde::{Deserialize, Serialize};

/// Represents the message sent from the website UI
#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequest {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    url: String,
    /// in seconds
    duration: Option<u16>,
    title: Option<String>,
    miniature_url: Option<String>,
}