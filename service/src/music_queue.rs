use crate::song::Song;

pub use common::MusicQueue;

pub trait MusicQueueExt {
    /// Returns (time-to-wait,position in queue)
    fn enqueue(&mut self, song: Song) -> (u32,usize);
    /// Returns song to be played now (if any)
    fn pull_next(&mut self) -> Option<Song>;
    fn get_currently_played_song(&self) -> Option<&Song>;
}

impl MusicQueueExt for MusicQueue {
    fn enqueue(&mut self, song: Song) -> (u32,usize) {
        let ttw = self.queue.iter().map(|s| s.duration.unwrap_or(0) as u32).sum();
        self.queue.push_back(song);
        let queue_pos = self.queue.len();
        (ttw,queue_pos)
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
