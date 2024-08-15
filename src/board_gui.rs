use termal::{codes, formatc, term_text::TermText};

use crate::{
    board::Board, draw_buffer::DrawBuffer, slice_2d::Slice2d, suit::Suit,
    vec2::Vec2,
};

#[derive(Default)]
pub struct GuiState {
    view_pos: Option<Vec2>,
}

impl GuiState {
    pub fn scroll_by(&mut self, off: impl Into<Vec2<isize>>) {
        if let Some(ref mut pos) = self.view_pos {
            *pos = pos.saturating_add_signed(off.into());
        }
    }

    pub fn center(&mut self) {
        self.view_pos = None;
    }
}

struct WinCrossPath {
    pre: char,
    post: char,
    line: char,
    line_offset: Vec2,
}

impl WinCrossPath {
    const fn new(
        pre: char,
        post: char,
        line: char,
        off: (usize, usize),
    ) -> Self {
        Self {
            pre,
            post,
            line,
            line_offset: Vec2::new(off.0, off.1),
        }
    }
}

const UR_CROSS: WinCrossPath = WinCrossPath::new(',', '\'', '/', (0, 2));
const DR_CROSS: WinCrossPath = WinCrossPath::new('\'', ',', '\\', (4, 2));
const D_CROSS: WinCrossPath = WinCrossPath::new(' ', ' ', '|', (2, 2));
const R_CROSS: WinCrossPath = WinCrossPath::new('-', '-', '-', (4, 1));

impl Board {
    pub fn draw(
        &self,
        gui: &mut GuiState,
        out: &mut DrawBuffer,
        space: Vec2,
        msg: &str,
    ) {
        let draw_size = self.size().cmul((4, 2)) + (1, 2);
        if draw_size.gt_or(space) {
            self.draw_scrollable(gui, out, space, msg);
            return;
        }

        Self::draw_msg(out, space, draw_size, msg);
        out.add_base(Self::center(space, draw_size - (0, 1)));

        let board = self.slice(..);

        Self::draw_grid(board, out);

        if let Some(pd) = self.win_pos() {
            self.draw_win_cross(board, out, pd);
        }

        gui.view_pos = None;

        self.draw_selected(gui, out);
    }

    fn draw_scrollable(
        &self,
        gui: &mut GuiState,
        out: &mut DrawBuffer,
        space: Vec2,
        msg: &str,
    ) {
        //   :   :
        // ..+---+..
        //   |   |
        // ..+---+..
        //   :   :
        //    msg
        let view_size = (space - (5, 4)).cdiv((4, 2)).cmin(self.size());
        if view_size.lt_or((1, 1)) {
            Self::draw_no_space(out);
            return;
        }

        let view_pos = gui
            .view_pos
            .unwrap_or_else(|| Self::center(self.size(), view_size));

        let bot_right = (view_pos.cmin(self.selected()) + view_size)
            .cmin(self.size())
            .cmax(self.selected() + (1, 1));
        let top_left = bot_right - view_size;

        Self::draw_msg(out, space, view_size.cmul((4, 2)) + (4, 3), msg);

        let board = self.slice(top_left..bot_right);
        out.add_base(
            Self::center(space - (0, 1), board.size().cmul((4, 2)) + (1, 1))
                - (2, 1),
        );
        self.draw_elipsis(board, out);

        out.add_base((2, 1));
        Self::draw_grid(board, out);

        if let Some(pd) = self.win_pos() {
            self.draw_win_cross(board, out, pd);
        }

        gui.view_pos = Some(top_left);
        self.draw_selected(gui, out);
    }

    fn draw_no_space(out: &mut DrawBuffer) {
        out.move_to((0, 0));
        *out += "Not enough space.";
    }

    fn draw_msg(
        out: &mut DrawBuffer,
        space: Vec2,
        mut draw_size: Vec2,
        msg: &str,
    ) {
        draw_size.y -= 1;
        let base = Self::center(space, draw_size);

        let msg_len = TermText::new(msg).display_char_cnt();
        let msgx = Self::center(space, (msg_len, 0)).x;

        out.move_to((0, base.y + draw_size.y));
        *out += codes::ERASE_TO_END;
        out.move_to((msgx, base.y + draw_size.y));
        *out += formatc!("{'_}{msg}");
    }

    fn draw_grid(board: Slice2d<'_, Suit>, out: &mut DrawBuffer) {
        *out += formatc!("{'_ gr}");
        for y in 0..board.size().y {
            out.move_to((0, y * 2));
            out.repeat(board.size().x, "+---");
            *out += '+';

            out.move_to((0, y * 2 + 1));
            for x in 0..board.size().x {
                *out += "| ";
                Self::draw_suit(out, board[(x, y)]);
                *out += formatc!(" {'gr}");
            }
            *out += '|';
        }

        out.move_to((0, board.size().y * 2));
        out.repeat(board.size().x, "+---");
        *out += '+';
    }

    fn draw_elipsis(&self, board: Slice2d<Suit>, out: &mut DrawBuffer) {
        *out += formatc!("{'_ gr}");

        let chr = if board.start().y != 0 { ':' } else { ' ' };
        out.move_to((2, 0));
        out.repeat(board.size().x, format!("{chr}   "));
        *out += chr;

        let s = if board.start().x != 0 { ".." } else { "  " };
        out.move_to((0, 1));
        out.repeat(board.size().y, formatc!("{s}\n\n{'ml ml}"));
        *out += s;

        let s = if board.end().x != self.size().x {
            ".."
        } else {
            "  "
        };
        out.move_to((3 + board.size().x * 4, 1));
        out.repeat(board.size().y, formatc!("{s}\n\n{'ml ml}"));
        *out += s;

        let chr = if board.end().y != self.size().y {
            ':'
        } else {
            ' '
        };
        out.move_to((2, 2 + board.size().y * 2));
        out.repeat(board.size().x, format!("{chr}   "));
        *out += chr;
    }

    fn draw_win_cross(
        &self,
        board: Slice2d<'_, Suit>,
        out: &mut DrawBuffer,
        pd: (Vec2, Vec2<isize>),
    ) {
        let path = match pd.1.tuple() {
            (-1, 1) => &UR_CROSS,
            (1, 1) => &DR_CROSS,
            (0, 1) => &D_CROSS,
            (1, 0) => &R_CROSS,
            _ => return,
        };
        self.draw_cell_cross(board, out, pd, path);
    }

    fn draw_cell_cross(
        &self,
        board: Slice2d<'_, Suit>,
        out: &mut DrawBuffer,
        (mut pos, dir): (Vec2, Vec2<isize>),
        path: &WinCrossPath,
    ) {
        let (color, _) = Self::get_color_char(self[pos]);
        *out += color;

        for i in (0..self.win_len()).rev() {
            if board.contains(pos) {
                out.move_to((pos - board.start()).cmul((4, 2)) + (1, 1));
                *out += formatc!("{}{'mr}{}", path.pre, path.post);
            }
            let new_pos = pos.wrapping_add_signed(dir);
            if i != 0 && (board.contains(new_pos) || board.contains(pos)) {
                out.move_to(
                    ((pos.signed() - board.start().signed()).cmul((4, 2))
                        + path.line_offset.signed())
                    .unsigned(),
                );
                *out += path.line;
            }
            pos = new_pos;
        }
    }

    fn draw_selected(&self, gui: &mut GuiState, out: &mut DrawBuffer) {
        let pos =
            (self.selected() - gui.view_pos.unwrap_or_default()).cmul((4, 2));

        out.move_to(pos);
        let (color, chr) = Self::get_color_char(self.on_turn());

        *out += format!("{color}{chr}---{chr}");
        out.move_to(pos + (0, 1));
        *out += formatc!("|{'mr mr mr}|");
        out.move_to(pos + (0, 2));
        *out += format!("{chr}---{chr}");
    }

    fn draw_suit(out: &mut DrawBuffer, suit: Suit) {
        match suit {
            Suit::None => *out += " ",
            Suit::Cross => *out += formatc!("{'b}X"),
            Suit::Circle => *out += formatc!("{'r}O"),
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
