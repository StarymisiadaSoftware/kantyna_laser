use serde::{Deserialize, Serialize};

/// Represents the message sent from the website UI
#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequest {
    pub url: String,
}
