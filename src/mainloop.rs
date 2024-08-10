use std::{
    io::{self, Write},
    time::Duration,
};

use termal::{
    codes, formatc, printc,
    raw::{
        self,
        events::{Event, KeyCode, Modifiers},
        Terminal,
    },
};

use crate::{
    board::Board,
    draw_buffer::DrawBuffer,
    err::{Error, Result},
    suit::Suit,
    vec2::Vec2,
};

const DEFAULT_MSG: &str = "\x1b[90mPress [h] to show help.";
const WAIT_TIME: Duration = Duration::from_millis(100);

pub struct Mainloop {
    board: Board,
    terminal: raw::Terminal,
    out: DrawBuffer,
    msg: String,
    persistant_msg: String,
    color: bool,
    redraw: bool,
    size: Vec2,
}

impl Mainloop {
    pub fn new(board: Board, color: bool) -> Self {
        Self {
            board,
            terminal: Terminal::new(),
            out: DrawBuffer::new(),
            msg: String::new(),
            persistant_msg: String::new(),
            color,
            redraw: true,
            size: (0, 0).into(),
        }
    }

    pub fn prepare() -> Result<()> {
        raw::enable_raw_mode()?;
        printc!("{'abuf e _e_ nocur}");
        _ = io::stdout().flush();
        Ok(())
    }

    pub fn restore() -> Result<()> {
        printc!("{'_abuf _nocur}");
        _ = io::stdout().flush();
        raw::disable_raw_mode()?;
        Ok(())
    }

    pub fn iterate(&mut self) -> Result<bool> {
        self.fetch_size()?;

        if self.redraw {
            self.draw();
        }

        self.update_msg();

        if !self.has_input()? {
            return Ok(true);
        }

        self.read_key()
    }

    pub fn run(&mut self) -> Result<()> {
        while self.iterate()? {}
        Ok(())
    }

    fn fetch_size(&mut self) -> Result<()> {
        let new_size = terminal_size()?;

        if new_size != self.size {
            self.out += formatc!("{'e _e_}");
            self.redraw = true;
        }

        self.size = new_size;
        Ok(())
    }

    fn draw(&mut self) {
        let msg = [self.msg.as_str(), &self.persistant_msg]
            .into_iter()
            .find(|a| !a.is_empty())
            .unwrap_or(DEFAULT_MSG);

        self.out.set_base((1, 1));
        self.board.draw(&mut self.out, self.size, msg);

        if self.color {
            self.out.clear_commit();
        } else {
            self.out.no_color_clear_commit();
        }

        self.msg.clear();

        self.redraw = false;
    }

    fn update_msg(&mut self) {
        if !self.persistant_msg.is_empty()
            && !self.persistant_msg.starts_with(codes::ESC)
        {
            self.persistant_msg.insert_str(0, codes::GRAY_FG);
        }
    }

    fn has_input(&self) -> Result<bool> {
        Ok(self.terminal.has_buffered_input()
            || raw::wait_for_stdin(WAIT_TIME)?)
    }

    fn read_key(&mut self) -> Result<bool> {
        let Event::KeyPress(key) = self.terminal.read()? else {
            return Ok(true);
        };

        match key.code {
            KeyCode::Up | KeyCode::Char('w') => {
                self.move_by((0, -1));
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.move_by((-1, 0));
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.move_by((0, 1));
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.move_by((1, 0));
            }
            KeyCode::Enter | KeyCode::Space | KeyCode::Char('0') => {
                self.play();
            }
            KeyCode::Char('u') => {
                self.board.undo();
            }
            KeyCode::Char('r') => {
                self.reset();
            }
            KeyCode::Char('q') => {
                return Ok(false);
            }
            KeyCode::Char('c') => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    return Err(Error::RageQuit);
                } else if key.modifiers.contains(Modifiers::SHIFT) {
                    self.persistant_msg.clear();
                } else {
                    self.toggle_color();
                }
            }
            KeyCode::Char('h') => {
                self.show_help();
            }
            _ => {
                return Ok(true);
            }
        }

        self.redraw = true;
        Ok(true)
    }

    fn move_by(&mut self, dif: impl Into<Vec2<isize>>) {
        self.board.set_selected(
            self.board.selected().saturating_add_signed(dif.into()),
        );
    }

    fn play(&mut self) {
        if let Err(e) = self.board.play() {
            self.msg += &formatc!("{'r}{e}{'_}");
        }
        match self.board.check_win() {
            None => {
                self.set_persistant_msg(formatc!("{'_}Draw!"));
            }
            Some(Suit::Circle) => {
                self.set_persistant_msg(formatc!("{'r}O {'_}Wins!\r"));
            }
            Some(Suit::Cross) => {
                self.set_persistant_msg(formatc!("{'b}X {'_}Wins!\r"));
            }
            _ => {
                return;
            }
        }

        self.board.inspect_mode();
    }

    fn set_persistant_msg(&mut self, s: impl AsRef<str>) {
        self.persistant_msg.clear();
        self.persistant_msg += s.as_ref();
    }

    fn reset(&mut self) {
        self.persistant_msg.clear();
        self.board.reset();
    }

    fn toggle_color(&mut self) {
        self.color = !self.color;
        if self.color {
            self.msg += "Colors enabled";
        } else {
            self.msg += "Colors disabled";
        }
    }

    fn show_help(&mut self) {
        self.persistant_msg.clear();
        self.persistant_msg +=
            "[Arrows/wasd]move [Enter/Space/0]play [q]quit \
            [r]restart [u]undo [h]help";
    }
}

fn terminal_size() -> Result<Vec2> {
    let size = raw::term_size()?;
    Ok((size.char_width, size.char_height).into())
}
