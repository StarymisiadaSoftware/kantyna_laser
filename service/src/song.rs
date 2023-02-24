use async_trait::async_trait;
pub use common::Song;
use thiserror::Error;


#[derive(Debug,Error)]
pub enum ValidationError {
    #[error("The length of your song exceeds the allowed limit!")]
    SongTooLong
}

#[derive(Debug,Error)]
pub enum YtDlpError {
    
}

#[async_trait]
pub trait SongExt {
    async fn load_from_ytdlp(&mut self) -> Result<(),YtDlpError>;
    fn validate(&self) -> Result<(),ValidationError>;
}

#[async_trait]
impl SongExt for Song {
    async fn load_from_ytdlp(&mut self) -> Result<(),YtDlpError> {
        Ok(())
    }
    fn validate(&self) -> Result<(),ValidationError> {
        // 20 minutes length limit
        if self.duration.unwrap_or(0) > 1200 {
            return Err(ValidationError::SongTooLong);
        }
        Ok(())
    }
}
