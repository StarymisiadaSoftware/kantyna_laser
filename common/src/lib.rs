use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct EnqueueRequest {
    pub url: String
}