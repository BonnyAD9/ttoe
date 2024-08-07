use std::ops::{Index, IndexMut};

use crate::{err::{Error, Result}, spot::Suit, vec2::Vec2};

pub struct Board {
    board: Vec<Suit>,
    width: usize,
    height: usize,
    on_turn: Suit,
    win_length: usize,
    selected: Vec2,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Vec::new();
        board.resize_with(width * height, || Suit::None);
        Self {
            board,
            width,
            height,
            on_turn: Suit::Cross,
            win_length: 5,
            selected: ((width - 1) / 2, (height - 1) / 2).into()
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn on_turn(&self) -> Suit {
        self.on_turn
    }

    pub fn selected(&self) -> Vec2 {
        self.selected
    }

    pub fn set_selected(&mut self, selected: Vec2) {
        self.selected = selected.clamp((0, 0), (self.width, self.height));
    }

    pub fn play(&mut self) -> Result<()> {
        let Vec2 { x, y } = self.selected;
        if x > self.width() || y > self.height() {
            return Err(Error::OutOfBounds);
        }

        if self[(x, y)] != Suit::None {
            return Err(Error::AlreadyPopulated);
        }

        self[(x, y)] = self.on_turn;
        self.on_turn = self.on_turn.oposite();

        Ok(())
    }

    pub fn check_win(&self) -> Option<Suit> {
        let mut draw = true;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let suit = self[(x, y)];
                if suit.is_none() {
                    draw = false;
                    continue;
                }

                if y + self.win_length <= self.height {
                    if x >= self.win_length && self.is_win((x, y), (-1, 1)) {
                        return Some(suit);
                    }
                    if self.is_win((x, y), (0, 1)) {
                        return Some(suit);
                    }
                    if x + self.win_length <= self.width && self.is_win((x, y), (1, 1)) {
                        return Some(suit);
                    }
                }
                if x + self.win_length <= self.width && self.is_win((x, y), (1, 0)) {
                    return Some(suit);
                }
            }
        }

        (!draw).then(|| Suit::None)
    }

    pub fn reset(&mut self) {
        for v in &mut self.board {
            *v = Suit::None;
        }
        self.on_turn = Suit::Cross;
        self.selected = ((self.width - 1) / 2, (self.height - 1) / 2).into();
    }

    fn is_win(&self, pos: impl Into<Vec2<usize>>, dir: impl Into<Vec2<isize>>) -> bool {
        let mut pos = pos.into();
        let dir = dir.into();
        let suit = self[pos];
        for _ in 0..self.win_length - 1 {
            pos = pos.wrapping_add_signed(dir);
            if self[pos] != suit {
                return false;
            }
        }
        return true;
    }
}

impl<T> Index<T> for Board where T: Into<Vec2> {
    type Output = Suit;

    fn index(&self, idx: T) -> &Self::Output {
        let Vec2 { x, y } = idx.into();
        if x > self.width || y > self.height {
            panic!("Index ({x}, {y}) out of range of ({}, {})", self.width, self.height);
        }
        &self.board[y * self.width + x]
    }
}

impl<T> IndexMut<T> for Board where T: Into<Vec2> {
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        let Vec2 { x, y } = idx.into();
        if x > self.width || y > self.height {
            panic!("Index ({x}, {y}) out of range of ({}, {})", self.width, self.height);
        }
        &mut self.board[y * self.width + x]
    }
}
