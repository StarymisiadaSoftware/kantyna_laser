use crate::song::Song;

#[derive(Default, Debug)]
pub struct MusicQueue {
    queue: Vec<Song>,
    currently_played: Option<Song>,
}