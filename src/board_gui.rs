use termal::{codes, formatc};

use crate::{board::Board, suit::Suit, vec2::Vec2};

impl Board {
    pub fn draw<F>(&self, out: &mut String, move_to: F, space: Vec2)
    where
        F: Fn(&mut String, Vec2),
    {
        let draw_size = self.size().cmul((4, 2)) + (1, 1).into();
        if draw_size.gt_or(space) {
            let msg = "Not enough space.";
            move_to(out, Self::center(space, (msg.len(), 1)));
            *out += &formatc!("{'r bold}{msg}");
            return;
        }

        let base = Self::center(space, draw_size);
        let move_to = |out: &mut String, pos| move_to(out, base + pos);

        *out += &formatc!("{'_ gr}");
        for y in 0..self.size().y {
            move_to(out, (0, y * 2).into());

            for _ in 0..self.size().x {
                *out += "+---";
            }
            *out += "+";
            move_to(out, (0, y * 2 + 1).into());

            for x in 0..self.size().x {
                *out += "| ";
                Self::draw_suit(out, self[(x, y)]);
                *out += &formatc!(" {'gr}");
            }
            *out += "|";
        }

        move_to(out, (0, self.size().y * 2).into());
        for _ in 0..self.size().x {
            *out += "+---";
        }
        *out += "+";

        let x = self.selected().x * 4;
        let y = self.selected().y * 2;

        move_to(out, (x, y).into());
        let color = match self.on_turn() {
            Suit::Circle => codes::RED_FG,
            Suit::Cross => codes::BLUE_FG,
            Suit::None => codes::WHITE_FG,
        };

        *out += &format!("{color}+---+");
        move_to(out, (x, y + 1).into());
        *out += "| ";
        Self::draw_suit(out, self[self.selected()]);
        *out += &format!(" {color}|");
        move_to(out, (x, y + 2).into());
        *out += "+---+";

        move_to(out, (0, self.size().y * 2 + 1).into());
    }

    fn draw_suit(out: &mut String, suit: Suit) {
        match suit {
            Suit::None => *out += " ",
            Suit::Cross => *out += &formatc!("{'b}X"),
            Suit::Circle => *out += &formatc!("{'r}O"),
        }
    }

    fn center(available: impl Into<Vec2>, required: impl Into<Vec2>) -> Vec2 {
        available.into().saturating_sub(required.into()) / 2
    }
}
