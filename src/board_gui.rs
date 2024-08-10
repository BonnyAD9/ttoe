use termal::{codes, formatc, term_text::TermText};

use crate::{board::Board, draw_buffer::DrawBuffer, suit::Suit, vec2::Vec2};

impl Board {
    pub fn draw(&self, out: &mut DrawBuffer, space: Vec2, msg: &str) {
        let mut draw_size = self.size().cmul((4, 2)) + (1, 2).into();
        if draw_size.gt_or(space) {
            let msg = "Not enough space.";
            let center = Self::center(space, (msg.len(), 1));
            out.move_to(center);
            *out += formatc!("{'r bold}{msg}");
            out.move_to(center + (0, 1).into());
            return;
        }

        draw_size.y -= 1;
        let base = Self::center(space, draw_size);

        let msg_len = TermText::new(msg).display_char_cnt();
        let msgx = Self::center(space, (msg_len, 0)).x;

        out.move_to((0, base.y + draw_size.y));
        *out += codes::ERASE_TO_END;
        out.move_to((msgx, base.y + draw_size.y));
        *out += &formatc!("{'_}{msg}");

        out.add_base(base);

        *out += &formatc!("{'_ gr}");
        for y in 0..self.size().y {
            out.move_to((0, y * 2));
            out.repeat(self.size().x, "+---");
            *out += "+";

            out.move_to((0, y * 2 + 1));
            for x in 0..self.size().x {
                *out += "| ";
                Self::draw_suit(out, self[(x, y)]);
                *out += formatc!(" {'gr}");
            }
            *out += '|';
        }

        out.move_to((0, self.size().y * 2));
        out.repeat(self.size().x, "+---");
        *out += "+";

        if let Some((pos, dir)) = self.win_pos() {
            let (color, _) = Self::get_color_char(self[pos]);

            match dir.tuple() {
                (-1, 1) => {
                    let mut pos = pos;
                    for _ in 1..self.win_len() {
                        out.move_to(pos.cmul((4, 2)) + (1, 1).into());
                        *out += format!("{color},");
                        out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                        *out += format!("{color}'");
                        out.move_to(pos.cmul((4, 2)) + (0, 2).into());
                        *out += format!("{color}/");
                        pos = pos.wrapping_add_signed(dir);
                    }
                    out.move_to(pos.cmul((4, 2)) + (1, 1).into());
                    *out += format!("{color},");
                    out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                    *out += format!("{color}'");
                }
                (1, 1) => {
                    let mut pos = pos;
                    for _ in 1..self.win_len() {
                        out.move_to(pos.cmul((4, 2)) + (1, 1).into());
                        *out += format!("{color}'");
                        out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                        *out += format!("{color},");
                        out.move_to(pos.cmul((4, 2)) + (4, 2).into());
                        *out += format!("{color}\\");
                        pos = pos.wrapping_add_signed(dir);
                    }
                    out.move_to(pos.cmul((4, 2)) + (1, 1).into());
                    *out += format!("{color}'");
                    out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                    *out += format!("{color},");
                }
                (0, 1) => {
                    let mut pos = pos;
                    for _ in 1..self.win_len() {
                        out.move_to(pos.cmul((4, 2)) + (2, 2).into());
                        *out += format!("{color}|");
                        pos = pos.wrapping_add_signed(dir);
                    }
                }
                (1, 0) => {
                    let mut pos = pos;
                    out.move_to(pos.cmul((4, 2)) + (1, 1).into());
                    *out += format!("{color}-");
                    for _ in 1..self.win_len() {
                        out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                        *out += format!("{color}---");
                        pos = pos.wrapping_add_signed(dir);
                    }
                    out.move_to(pos.cmul((4, 2)) + (3, 1).into());
                    *out += format!("{color}-");
                }
                _ => {}
            }
        }

        let x = self.selected().x * 4;
        let y = self.selected().y * 2;

        out.move_to((x, y));
        let (color, chr) = Self::get_color_char(self.on_turn());

        *out += format!("{color}{chr}---{chr}");
        out.move_to((x, y + 1));
        *out += '|';
        out.move_to((x + 4, y + 1));
        *out += '|';
        out.move_to((x, y + 2));
        *out += &format!("{chr}---{chr}");
    }

    fn draw_suit(out: &mut DrawBuffer, suit: Suit) {
        match suit {
            Suit::None => *out += " ",
            Suit::Cross => *out += &formatc!("{'b}X"),
            Suit::Circle => *out += &formatc!("{'r}O"),
        }
    }

    fn center(available: impl Into<Vec2>, required: impl Into<Vec2>) -> Vec2 {
        available.into().saturating_sub(required.into()) / 2
    }

    fn get_color_char(suit: Suit) -> (&'static str, char) {
        match suit {
            Suit::Circle => (codes::RED_FG, 'o'),
            Suit::Cross => (codes::BLUE_FG, 'x'),
            Suit::None => (codes::WHITE_FG, '+'),
        }
    }
}
