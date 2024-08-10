use std::{
    io::{stdout, Write},
    ops::{AddAssign, SubAssign},
};

use termal::{printc, term_text::TermText};

use crate::{append_str::AppendStr, vec2::Vec2};

#[derive(Default)]
pub struct DrawBuffer {
    buf: String,
    base: Vec2,
}

impl DrawBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_base(&mut self, change: impl Into<Vec2>) {
        self.base += change.into();
    }

    pub fn set_base(&mut self, base: impl Into<Vec2>) {
        self.base = base.into();
    }

    pub fn draw(&mut self, s: impl AppendStr) {
        s.append_to(&mut self.buf);
    }

    pub fn move_to(&mut self, pos: impl Into<Vec2>) {
        let pos = pos.into() + self.base;
        self.draw(termal::move_to!(pos.x, pos.y));
    }

    pub fn repeat(&mut self, cnt: usize, s: impl AppendStr) {
        self.buf.reserve(cnt * s.append_len());
        for _ in 0..cnt {
            self.draw(&s);
        }
    }

    pub fn move_left(&mut self, amount: usize) {
        if amount != 0 {
            self.draw(termal::move_left!(amount));
        }
    }

    pub fn move_right(&mut self, amount: usize) {
        if amount != 0 {
            self.draw(termal::move_right!(amount));
        }
    }

    pub fn move_down(&mut self, amount: usize) {
        if amount != 0 {
            self.draw(termal::move_down!(amount));
        }
    }

    pub fn move_up(&mut self, amount: usize) {
        if amount != 0 {
            self.draw(termal::move_up!(amount));
        }
    }

    pub fn clear_commit(&mut self) {
        printc!("{'_}{}", self.buf);
        _ = stdout().flush();
        self.buf.clear();
    }

    pub fn no_color_clear_commit(&mut self) {
        let mut out = String::new();
        for span in TermText::new(&self.buf)
            .spans()
            .filter(|a| !a.is_control() || !a.text().ends_with('m'))
        {
            out += span.text();
        }
        printc!("{'_}{}", out);
        _ = stdout().flush();
        self.buf.clear();
    }
}

impl<T> AddAssign<T> for DrawBuffer
where
    T: AppendStr,
{
    fn add_assign(&mut self, rhs: T) {
        self.draw(rhs);
    }
}

impl AddAssign<Vec2> for DrawBuffer {
    fn add_assign(&mut self, rhs: Vec2) {
        self.move_right(rhs.x);
        self.move_down(rhs.y);
    }
}

impl AddAssign<Vec2<isize>> for DrawBuffer {
    fn add_assign(&mut self, rhs: Vec2<isize>) {
        if rhs.x >= 0 {
            self.move_right(rhs.x as usize);
        } else {
            self.move_left(rhs.x.unsigned_abs());
        }
        if rhs.y >= 0 {
            self.move_down(rhs.y as usize);
        } else {
            self.move_up(rhs.y.unsigned_abs());
        }
    }
}

impl SubAssign<Vec2> for DrawBuffer {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.move_left(rhs.x);
        self.move_up(rhs.y);
    }
}

impl SubAssign<Vec2<isize>> for DrawBuffer {
    fn sub_assign(&mut self, rhs: Vec2<isize>) {
        if rhs.x < 0 {
            self.move_right(rhs.x as usize);
        } else {
            self.move_left(rhs.x.unsigned_abs());
        }
        if rhs.y < 0 {
            self.move_down(rhs.y as usize);
        } else {
            self.move_up(rhs.y.unsigned_abs());
        }
    }
}
