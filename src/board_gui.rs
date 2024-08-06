use termal::formatc;

use crate::{board::Board, spot::Suit};

impl Board {
    pub fn draw<F>(&self, out: &mut String, move_to: F) where F: Fn(&mut String, usize, usize) {
        *out += &formatc!("{'_ gr}");
        for y in 0..self.height() {
            move_to(out, 0, y * 2);

            for _ in 0..self.width() {
                *out += "+---";
            }
            *out += "+";
            move_to(out, 0, y * 2 + 1);

            for x in 0..self.width() {
                match self[(x, y)] {
                    Suit::None => *out += "|   ",
                    Suit::Cross => *out += &formatc!("| {'b}X {'gr}"),
                    Suit::Circle => *out += &formatc!("| {'r}O {'gr}"),
                }
            }
            *out += "|";
        }

        move_to(out, 0, self.height() * 2);
        for _ in 0..self.width() {
            *out += "+---";
        }
        *out += "+";
    }
}
