use std::ops::Index;

use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Slice2d<'a, T> {
    slice: &'a [T],
    whole_size: Vec2,
    start: Vec2,
    size: Vec2,
}

impl<'a, T> Slice2d<'a, T> {
    pub fn new(
        slice: &'a [T],
        whole_size: Vec2,
        start: Vec2,
        size: Vec2,
    ) -> Self {
        Self {
            slice,
            whole_size,
            start,
            size,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.start + self.size
    }

    pub fn contains(&self, idx: Vec2) -> bool {
        idx.ge_and(self.start()) && idx.lt_and(self.end())
    }
}

impl<'a, T, I> Index<I> for Slice2d<'a, T>
where
    I: Into<Vec2>,
{
    type Output = T;

    fn index(&self, idx: I) -> &Self::Output {
        let idx = idx.into();
        if idx.gt_or(self.size) {
            panic!("Index {idx} out of range of {}", self.size);
        }
        &self.slice
            [(idx.y + self.start.y) * self.whole_size.x + idx.x + self.start.x]
    }
}
