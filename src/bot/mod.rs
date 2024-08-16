use crate::{board::Board, vec2::Vec2};

mod bias_rng;
mod pure_rng;
mod smart_rng;
mod brute;

pub use self::{bias_rng::*, pure_rng::*, smart_rng::*, brute::*};

pub trait Bot {
    fn play(&mut self, board: &Board, last: Option<Vec2>) -> Vec2;
}
