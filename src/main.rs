use std::{
    io::{stdout, Write},
    process::ExitCode,
    time::Duration,
};

use board::Board;
use err::Result;
use suit::Suit;
use termal::{
    codes, eprintcln, formatc, printc,
    raw::{
        self,
        events::{Event, KeyCode, Modifiers},
    },
};
use vec2::Vec2;

mod board;
mod board_gui;
mod err;
mod suit;
mod vec2;
mod vec2_range;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            print!(
                "{}{}",
                codes::DISABLE_ALTERNATIVE_BUFFER,
                codes::SHOW_CURSOR
            );
            eprintcln!("{'r}error: {e}");
            _ = raw::disable_raw_mode();
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    raw::enable_raw_mode()?;

    let mut board = Board::new((20, 20));
    let mut terminal = raw::Terminal::new();
    let mut out = String::new();
    let mut msg = String::new();

    let default_msg = formatc!("{'gr}Press [h] to show help.");
    let mut persistant_msg = String::new();

    out += codes::ENABLE_ALTERNATIVE_BUFFER;
    out += codes::ERASE_ALL;
    out += codes::ERASE_SCREEN;
    out += codes::HIDE_CURSOR;
    print!("{}", out);

    let mut size = terminal_size()?;

    let mut redraw = true;

    loop {
        out.clear();
        let new_size = terminal_size()?;
        if new_size != size {
            out += codes::ERASE_ALL;
            out += codes::ERASE_SCREEN;
            redraw = true;
        }
        size = new_size;

        if redraw {
            let msg = [msg.as_str(), &persistant_msg]
                .into_iter()
                .find(|a| !a.is_empty())
                .unwrap_or(&default_msg);
            board.draw(
                &mut out,
                |s, Vec2 { x, y }| *s += &termal::move_to!(x + 1, y + 1),
                size - (0, 1).into(),
                msg,
            );
            printc!("{out}");
            _ = stdout().flush();
            redraw = false;
        }
        msg.clear();
        if !persistant_msg.is_empty()
            && !persistant_msg.starts_with(codes::ESC)
        {
            persistant_msg.insert_str(0, codes::GRAY_FG);
        }

        if !terminal.has_buffered_input()
            && !raw::wait_for_stdin(Duration::from_millis(100))?
        {
            continue;
        }

        let Event::KeyPress(key) = terminal.read()? else {
            continue;
        };

        match key.code {
            KeyCode::Up | KeyCode::Char('w') => {
                board.set_selected(board.selected().saturating_sub((0, 1)));
            }
            KeyCode::Left | KeyCode::Char('a') => {
                board.set_selected(board.selected().saturating_sub((1, 0)));
            }
            KeyCode::Down | KeyCode::Char('s') => {
                board.set_selected(board.selected() + (0, 1).into());
            }
            KeyCode::Right | KeyCode::Char('d') => {
                board.set_selected(board.selected() + (1, 0).into());
            }
            KeyCode::Enter | KeyCode::Space | KeyCode::Char('0') => {
                if let Err(e) = board.play() {
                    msg += &formatc!("{'r}error: {'_}{e}");
                }
                match board.check_win() {
                    None => {
                        persistant_msg.clear();
                        persistant_msg += &formatc!("{'_}Draw!");
                        board.inspect_mode();
                    }
                    Some(Suit::Circle) => {
                        persistant_msg.clear();
                        persistant_msg += &formatc!("{'r}O {'_}Wins!\r");
                        board.inspect_mode();
                    }
                    Some(Suit::Cross) => {
                        persistant_msg.clear();
                        persistant_msg += &formatc!("{'b}X {'_}Wins!\r");
                        board.inspect_mode();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('u') => {
                board.undo();
            }
            KeyCode::Char('r') => {
                persistant_msg.clear();
                board.reset();
            }
            KeyCode::Char('q') => {
                break;
            }
            KeyCode::Char('c') => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    break;
                }
                persistant_msg.clear();
            }
            KeyCode::Char('h') => {
                persistant_msg.clear();
                persistant_msg += "[Arrows/wasd]move [Enter/Space/0]play \
                [q]quit [r]restart [u]undo [h]help";
            }
            _ => {
                continue;
            }
        }

        redraw = true;
    }

    raw::disable_raw_mode()?;
    print!(
        "{}{}",
        codes::DISABLE_ALTERNATIVE_BUFFER,
        codes::SHOW_CURSOR
    );

    Ok(())
}

fn terminal_size() -> Result<Vec2> {
    let size = raw::term_size()?;
    Ok((size.char_width, size.char_height).into())
}
