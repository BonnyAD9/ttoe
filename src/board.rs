use std::ops::{Index, IndexMut, RangeBounds};

use crate::{
    err::{Error, Result},
    slice_2d::Slice2d,
    suit::Suit,
    vec2::Vec2,
};

pub struct Board {
    board: Vec<Suit>,
    size: Vec2,
    on_turn: Suit,
    win_len: usize,
    selected: Vec2,
    last: Vec2,
    win_pos: Option<(Vec2, Vec2<isize>)>,
}

impl Board {
    pub fn new(size: impl Into<Vec2>, win_len: usize) -> Self {
        let mut board = Vec::new();
        let size = size.into();
        board.resize_with(size.prod(), || Suit::None);
        Self {
            board,
            size,
            on_turn: Suit::Cross,
            win_len,
            selected: (size - (1, 1)) / 2,
            last: (0, 0).into(),
            win_pos: None,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn on_turn(&self) -> Suit {
        self.on_turn
    }

    pub fn inspect_mode(&mut self) {
        self.on_turn = Suit::None
    }

    pub fn selected(&self) -> Vec2 {
        self.selected
    }

    pub fn set_selected(&mut self, selected: Vec2) {
        self.selected = selected.clamp((0, 0), self.size - (1, 1));
    }

    pub fn win_pos(&self) -> Option<(Vec2, Vec2<isize>)> {
        self.win_pos
    }

    pub fn win_len(&self) -> usize {
        self.win_len
    }

    pub fn play(&mut self) -> Result<()> {
        if self.on_turn == Suit::None {
            return Ok(());
        }

        let sel = self.selected;

        if self[sel] != Suit::None {
            return Err(Error::AlreadyPopulated);
        }

        self.last = sel;
        self[sel] = self.on_turn;
        self.on_turn = self.on_turn.oposite();

        Ok(())
    }

    pub fn check_win(&mut self) -> Option<Suit> {
        let mut draw = true;
        for pos in Vec2::new(0, 0).to(self.size) {
            let suit = self[pos];
            if suit.is_none() {
                draw = false;
                continue;
            }

            if pos.y + self.win_len <= self.size.y {
                if pos.x >= self.win_len && self.is_win(pos, (-1, 1)) {
                    return Some(suit);
                }
                if self.is_win(pos, (0, 1)) {
                    return Some(suit);
                }
                if pos.x + self.win_len <= self.size.x
                    && self.is_win(pos, (1, 1))
                {
                    return Some(suit);
                }
            }
            if pos.x + self.win_len <= self.size.x && self.is_win(pos, (1, 0))
            {
                return Some(suit);
            }
        }

        (!draw).then_some(Suit::None)
    }

    pub fn reset(&mut self) {
        for v in &mut self.board {
            *v = Suit::None;
        }
        self.on_turn = Suit::Cross;
        self.win_pos = None;
    }

    pub fn undo(&mut self) {
        let last = self.last;
        if self[last] != Suit::None {
            self[last] = Suit::None;
            self.on_turn = self.on_turn.oposite();
        }
    }

    pub fn slice(&self, range: impl RangeBounds<Vec2>) -> Slice2d<'_, Suit> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s + (1, 1),
            std::ops::Bound::Unbounded => (0, 0).into(),
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(e) => *e + (1, 1),
            std::ops::Bound::Excluded(e) => *e,
            std::ops::Bound::Unbounded => self.size(),
        };

        Slice2d::new(&self.board, self.size, start, end - start)
    }

    fn is_win(
        &mut self,
        pos: impl Into<Vec2<usize>>,
        dir: impl Into<Vec2<isize>>,
    ) -> bool {
        let mut pos = pos.into();
        let pos1 = pos;
        let dir = dir.into();
        let suit = self[pos];
        for _ in 0..self.win_len - 1 {
            pos = pos.wrapping_add_signed(dir);
            if self[pos] != suit {
                return false;
            }
        }
        self.win_pos = Some((pos1, dir));
        true
    }
}

impl<T> Index<T> for Board
where
    T: Into<Vec2>,
{
    type Output = Suit;

    fn index(&self, idx: T) -> &Self::Output {
        let idx = idx.into();
        if idx.gt_or(self.size) {
            panic!("Index {idx} out of range of {}", self.size);
        }
        &self.board[idx.y * self.size.x + idx.x]
    }
}

impl<T> IndexMut<T> for Board
where
    T: Into<Vec2>,
{
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        let idx = idx.into();
        if idx.gt_or(self.size) {
            panic!("Index {idx} out of range of {}", self.size);
        }
        &mut self.board[idx.y * self.size.x + idx.x]
    }
}
