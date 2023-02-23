use crate::song::Song;

pub use common::MusicQueue;

pub trait MusicQueueExt {
    fn enqueue(&mut self, song: Song);
    /// Returns song to be played now (if any)
    fn pull_next(&mut self) -> Option<Song>;
    fn get_currently_played_song(&self) -> Option<&Song> ;
}

impl MusicQueueExt for MusicQueue {
    fn enqueue(&mut self, song: Song) {
        self.queue.push_back(song);
    }
    fn pull_next(&mut self) -> Option<Song> {
        let cp = self.queue.pop_front();
        self.currently_played = cp.clone();
        cp
    }
    fn get_currently_played_song(&self) -> Option<&Song> {
        self.currently_played.as_ref()
    }
}