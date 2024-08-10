use termal::{codes, formatc, term_text::TermText};

use crate::{board::Board, draw_buffer::DrawBuffer, suit::Suit, vec2::Vec2};

impl Board {
    pub fn draw(&self, out: &mut DrawBuffer, space: Vec2, msg: &str) {
        let draw_size = self.size().cmul((4, 2)) + (1, 2).into();
        if draw_size.gt_or(space) {
            Self::draw_no_space(out, space);
            return;
        }

        let base = Self::draw_msg(out, space, draw_size, msg);
        out.add_base(base);

        self.draw_grid(out);

        if let Some(pd) = self.win_pos() {
            self.draw_win_cross(out, pd);
        }

        self.draw_selected(out);
    }

    fn draw_no_space(out: &mut DrawBuffer, space: Vec2) {
        let msg = "Not enough space.";
        let center = Self::center(space, (msg.len(), 1));
        out.move_to(center);
        *out += formatc!("{'r bold}{msg}");
        out.move_to(center + (0, 1).into());
    }

    fn draw_msg(
        out: &mut DrawBuffer,
        space: Vec2,
        mut draw_size: Vec2,
        msg: &str,
    ) -> Vec2 {
        draw_size.y -= 1;
        let base = Self::center(space, draw_size);

        let msg_len = TermText::new(msg).display_char_cnt();
        let msgx = Self::center(space, (msg_len, 0)).x;

        out.move_to((0, base.y + draw_size.y));
        *out += codes::ERASE_TO_END;
        out.move_to((msgx, base.y + draw_size.y));
        *out += &formatc!("{'_}{msg}");

        base
    }

    fn draw_grid(&self, out: &mut DrawBuffer) {
        *out += &formatc!("{'_ gr}");
        for y in 0..self.size().y {
            out.move_to((0, y * 2));
            out.repeat(self.size().x, "+---");
            *out += '+';

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
        *out += '+';
    }

    fn draw_win_cross(&self, out: &mut DrawBuffer, pd: (Vec2, Vec2<isize>)) {
        match pd.1.tuple() {
            (-1, 1) => {
                self.draw_cell_cross(out, pd, ',', '\'', '/', (0, 2));
            }
            (1, 1) => {
                self.draw_cell_cross(out, pd, '\'', ',', '\\', (4, 2));
            }
            (0, 1) => {
                self.draw_cell_cross(out, pd, ' ', ' ', '|', (2, 2));
            }
            (1, 0) => {
                self.draw_cell_cross(out, pd, '-', '-', '-', (4, 1));
            }
            _ => {}
        }
    }

    fn draw_cell_cross(
        &self,
        out: &mut DrawBuffer,
        (mut pos, dir): (Vec2, Vec2<isize>),
        pre: char,
        post: char,
        line: char,
        line_offset: impl Into<Vec2>,
    ) {
        let line_offset = line_offset.into();
        let (color, _) = Self::get_color_char(self[pos]);
        *out += color;

        for i in (0..self.win_len()).rev() {
            out.move_to(pos.cmul((4, 2)) + (1, 1).into());
            *out += formatc!("{pre}{'mr}{post}");
            if i != 0 {
                out.move_to(pos.cmul((4, 2)) + line_offset);
                *out += line;
            }
            pos = pos.wrapping_add_signed(dir);
        }
    }

    fn draw_selected(&self, out: &mut DrawBuffer) {
        let x = self.selected().x * 4;
        let y = self.selected().y * 2;

        out.move_to((x, y));
        let (color, chr) = Self::get_color_char(self.on_turn());

        *out += format!("{color}{chr}---{chr}");
        out.move_to((x, y + 1));
        *out += formatc!("|{'mr mr mr}|");
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
            Suit::None => (codes::WHITE_FG, '#'),
        }
    }
}
