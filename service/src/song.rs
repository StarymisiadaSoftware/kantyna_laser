use anyhow::Result;
use async_trait::async_trait;
pub use common::Song;

#[async_trait]
pub trait SongExt {
    async fn load_from_ytdlp(&mut self) -> Result<()>;
    fn validate(&self) -> Result<()>;
}

#[async_trait]
impl SongExt for Song {
    async fn load_from_ytdlp(&mut self) -> Result<()> {
        Ok(())
    }
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
