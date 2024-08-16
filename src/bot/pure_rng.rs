use rand::{seq::SliceRandom, thread_rng};

use crate::{board::Board, vec2::Vec2};

use super::Bot;

pub struct PureRng();

impl PureRng {
    pub fn new() -> Self {
        Self()
    }
}

impl Bot for PureRng {
    fn play(&mut self, board: &Board, last: Option<Vec2>) -> Vec2 {
        let mut empty = vec![];

        for pos in Vec2::new(0, 0).to(board.size()) {
            if board[pos].is_none() {
                empty.push(pos);
            }
        }

        empty[..].choose(&mut thread_rng()).copied().unwrap_or_else(|| board.size().saturating_sub((1, 1)) / 2)
    }
}
