use crate::song::Song;
use std::collections::VecDeque;
#[derive(Default, Debug)]
pub struct MusicQueue {
    queue: VecDeque<Song>,
    currently_played: Option<Song>,
}

impl MusicQueue {
    pub fn enqueue(&mut self, song: Song) {
        self.queue.push_back(song);
    }
    /// Returns song to be played now (if any)
    pub fn pull_next(&mut self) -> Option<Song> {
        let cp = self.queue.pop_front();
        self.currently_played = cp.clone();
        cp
    }
    pub fn get_currently_played_song(&self) -> Option<&Song> {
        self.currently_played.as_ref()
    }
}