use crate::vec2::Vec2;

pub struct Vec2Range<T = usize> {
    start: Vec2<T>,
    x: T,
    end: Vec2<T>,
}

impl<T> Vec2Range<T> {
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self
    where
        T: Copy,
    {
        Vec2Range {
            start,
            x: start.x,
            end,
        }
    }
}

impl Iterator for Vec2Range<usize> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.end.x {
            if self.x == self.start.x {
                return None;
            }
            self.x = self.start.x;
            self.start.y += 1;
        }

        if self.start.y >= self.end.y {
            return None;
        }

        self.x += 1;

        Some((self.x - 1, self.start.y).into())
    }
}
