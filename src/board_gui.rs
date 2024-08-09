use termal::{codes, formatc, term_text::TermText};

use crate::{board::Board, suit::Suit, vec2::Vec2};

impl Board {
    pub fn draw<F>(&self, out: &mut String, move_to: F, space: Vec2, msg: &str)
    where
        F: Fn(&mut String, Vec2),
    {
        let mut draw_size = self.size().cmul((4, 2)) + (1, 2).into();
        if draw_size.gt_or(space) {
            let msg = "Not enough space.";
            let center = Self::center(space, (msg.len(), 1));
            move_to(out, center);
            *out += &formatc!("{'r bold}{msg}");
            move_to(out, center + (0, 1).into());
            return;
        }

        draw_size.y -= 1;
        let base = Self::center(space, draw_size);

        let msg_len = TermText::new(msg).display_char_cnt();
        let msgx = Self::center(space, (msg_len, 0)).x;

        move_to(out, (0, base.y + draw_size.y).into());
        *out += codes::ERASE_TO_END;
        move_to(out, (msgx, base.y + draw_size.y).into());
        *out += &formatc!("{'_}{msg}");

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
        let (color, chr) = match self.on_turn() {
            Suit::Circle => (codes::RED_FG, 'o'),
            Suit::Cross => (codes::BLUE_FG, 'x'),
            Suit::None => (codes::WHITE_FG, '+'),
        };

        *out += &format!("{color}{chr}---{chr}");
        move_to(out, (x, y + 1).into());
        *out += "| ";
        Self::draw_suit(out, self[self.selected()]);
        *out += &format!(" {color}|");
        move_to(out, (x, y + 2).into());
        *out += &format!("{chr}---{chr}");
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
