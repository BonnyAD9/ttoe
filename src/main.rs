use std::{
    io::{stdout, Write},
    process::ExitCode,
};

use board::Board;
use err::Result;
use suit::Suit;
use termal::{
    codes, eprintcln, formatc,
    raw::{
        self,
        events::{Event, KeyCode},
    },
};

mod board;
mod board_gui;
mod err;
mod suit;
mod vec2;

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

    let mut board = Board::new(20, 20);
    let mut terminal = raw::Terminal::new();
    let mut out = String::new();
    let mut msg = String::new();

    out += codes::ENABLE_ALTERNATIVE_BUFFER;
    out += codes::ERASE_ALL;
    out += codes::ERASE_SCREEN;
    out += codes::HIDE_CURSOR;
    print!("{}", out);

    loop {
        out.clear();
        board.draw(&mut out, |s, x, y| *s += &termal::move_to!(x + 1, y + 1));
        print!("{out}{}{msg}", codes::ERASE_TO_END);
        _ = stdout().flush();
        msg.clear();

        let Event::KeyPress(key) = terminal.read()? else {
            continue;
        };

        match key.code {
            KeyCode::Up => {
                board.set_selected(board.selected() - (0, 1).into());
            }
            KeyCode::Right => {
                board.set_selected(board.selected() + (1, 0).into());
            }
            KeyCode::Down => {
                board.set_selected(board.selected() + (0, 1).into());
            }
            KeyCode::Left => {
                board.set_selected(board.selected() - (1, 0).into());
            }
            KeyCode::Enter => {
                if let Err(e) = board.play() {
                    msg += &formatc!("{'r}error: {'_}{e}");
                }
                match board.check_win() {
                    None => {
                        msg += &formatc!("{'_}Draw!");
                        board.inspect_mode();
                    }
                    Some(Suit::Circle) => {
                        msg += &formatc!("{'r}O {'_}Wins!\r");
                        board.inspect_mode();
                    }
                    Some(Suit::Cross) => {
                        msg += &formatc!("{'b}X {'_}Wins!\r");
                        board.inspect_mode();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('u') => {
                board.undo();
            }
            KeyCode::Char('r') => {
                board.reset();
            }
            KeyCode::Char('q') => {
                break;
            }
            KeyCode::Char('h') => {
                msg += &formatc!(
                    "{'_}[↑→↓←]move [Enter]play [q]quit [r]restart [u]undo \
                    [h]help"
                );
            }
            _ => {}
        }
    }

    raw::disable_raw_mode()?;
    print!(
        "{}{}",
        codes::DISABLE_ALTERNATIVE_BUFFER,
        codes::SHOW_CURSOR
    );

    Ok(())
}
