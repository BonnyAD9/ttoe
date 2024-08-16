use rand::{seq::SliceRandom, thread_rng};

use crate::{board::Board, vec2::Vec2};

use super::Bot;

pub struct SmartRng();

impl SmartRng {
    pub fn new() -> Self {
        Self()
    }
}

impl Bot for SmartRng {
    fn play(&mut self, board: &Board, last: Option<Vec2>) -> Vec2 {
        let mut places = vec![];

        for pos in Vec2::new(0, 0).to(board.size()) {
            if !board[pos].is_none() {
                continue;
            }

            if pos.surround().any(|p| p.lt_and(board.size()) && !board[p].is_none()) {
                places.push(pos);
            }
        }

        places[..].choose(&mut thread_rng()).copied().unwrap_or_else(|| board.size().saturating_sub((1, 1)) / 2)
    }
}
