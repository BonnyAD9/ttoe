use termal::{codes, formatc};

use crate::{board::Board, spot::Suit, vec2::Vec2};

impl Board {
    pub fn draw<F>(&self, out: &mut String, move_to: F) where F: Fn(&mut String, usize, usize) {
        fn draw_suit(out: &mut String, suit: Suit) {
            match suit {
                Suit::None => *out += " ",
                Suit::Cross => *out += &formatc!("{'b}X"),
                Suit::Circle => *out += &formatc!("{'r}O"),
            }
        }

        *out += &formatc!("{'_ gr}");
        for y in 0..self.height() {
            move_to(out, 0, y * 2);

            for _ in 0..self.width() {
                *out += "+---";
            }
            *out += "+";
            move_to(out, 0, y * 2 + 1);

            for x in 0..self.width() {
                *out += "| ";
                draw_suit(out, self[(x, y)]);
                *out += &formatc!(" {'gr}");
            }
            *out += "|";
        }

        move_to(out, 0, self.height() * 2);
        for _ in 0..self.width() {
            *out += "+---";
        }
        *out += "+";

        let x = self.selected().x * 4;
        let y = self.selected().y * 2;

        move_to(out, x, y);
        let color = match self.on_turn() {
            Suit::Circle => codes::RED_FG,
            Suit::Cross => codes::BLUE_FG,
            Suit::None => codes::WHITE_FG,
        };

        *out += &format!("{color}+---+");
        move_to(out, x, y + 1);
        *out += "| ";
        draw_suit(out, self[self.selected()]);
        *out += &format!(" {color}|");
        move_to(out, x, y + 2);
        *out += &format!("+---+");

        move_to(out, 0, self.height() * 2 + 1);
    }
}
