use async_trait::async_trait;
pub use common::Song;
use serde::Deserialize;
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("The length of your song exceeds the allowed limit!")]
    SongTooLong,
}

#[derive(Debug, Error)]
pub enum YtDlpError {
    #[error("Could not run 'yt-dlp': {0}")]
    SpawningError(#[from] std::io::Error),
    #[error("Could not deserialize output from 'yt-dlp': {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("'yt-dlp' couldn't extract info about song: {0}")]
    ExtractionError(String),
}

#[derive(Debug, Deserialize)]
struct YtDlpJson {
    pub title: String,
    pub duration: u16,
    pub thumbnail: String,
}

#[async_trait]
pub trait SongExt {
    async fn load_from_ytdlp(&mut self) -> Result<(), YtDlpError>;
    fn validate(&self) -> Result<(), ValidationError>;
}

#[async_trait]
impl SongExt for Song {
    async fn load_from_ytdlp(&mut self) -> Result<(), YtDlpError> {
        let mut yt_dlp = Command::new("yt-dlp");
        yt_dlp.arg("--dump-json");
        yt_dlp.arg(&self.url);
        let output = yt_dlp.output().await?;
        if !output.status.success() {
            return Err(YtDlpError::ExtractionError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
        let json = serde_json::from_slice::<YtDlpJson>(&output.stdout)?;
        self.miniature_url = Some(json.thumbnail);
        self.duration = Some(json.duration);
        self.title = Some(json.title);
        Ok(())
    }
    fn validate(&self) -> Result<(), ValidationError> {
        // 20 minutes length limit
        if self.duration.unwrap_or(0) > 1200 {
            return Err(ValidationError::SongTooLong);
        }
        Ok(())
    }
}
