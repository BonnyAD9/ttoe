use rand::{thread_rng, Rng, RngCore};

use crate::{board::Board, vec2::Vec2};

use super::Bot;

pub struct BiasRng();

impl BiasRng {
    pub fn new() -> Self {
        Self()
    }
}

impl Bot for BiasRng {
    fn play(&mut self, board: &Board, _last: Option<Vec2>) -> Vec2 {
        let mut rng = thread_rng();

        let mut pos = Vec2::new(rng.gen_range(0..board.size().x), rng.gen_range(0..board.size().y));
        while !board[pos].is_none() {
            pos = (pos + (1, 0)).mod_size(board.size());
        }

        pos
    }
}
