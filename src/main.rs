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
            print!("{}", codes::DISABLE_ALTERNATIVE_BUFFER);
            eprintcln!("{'r}error: {e}");
            _ = raw::disable_raw_mode();
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    /*printcln!("{'gr}┌───┬───┬───┐");
    printcln!("│ {'b}X {'gr}│ {'b}X {'gr}│ {'b}X {'gr}│");
    printcln!("├───╆━━━╅───┤");
    printcln!("│ {'r}O {'gr}┃{'w}[{'r}O{'w}]{'gr}┃ {'r}O {'gr}│");
    printcln!("└───┺━━━┹───┘{'_}");
    printcln!("{'gr}+---+---+---+");
    printcln!("| {'b}X {'gr}| {'b}X {'gr}| {'b}X {'gr}|");
    printcln!("+---{'w}+---+{'gr}---+");
    printcln!("| {'r}O {'w}| {'r}O {'w}| {'r}O {'gr}|");
    printcln!("+---{'w}+---+{'gr}---+{'_}");*/

    raw::enable_raw_mode()?;

    let mut board = Board::new(20, 20);
    let mut terminal = raw::Terminal::new();
    let mut out = String::new();
    let mut msg = String::new();
    let mut endgame = false;

    out += codes::ENABLE_ALTERNATIVE_BUFFER;
    out += codes::ERASE_ALL;
    out += codes::ERASE_SCREEN;
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

        if key.code == KeyCode::Char('q') {
            break;
        } else if endgame {
            endgame = false;
            board.reset();
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
                        msg += &formatc!("{out}{'_}Draw!");
                        endgame = true;
                    }
                    Some(Suit::Circle) => {
                        msg += &formatc!("{out}{'r}O {'_}Wins!\r");
                        endgame = true;
                    }
                    Some(Suit::Cross) => {
                        msg += &formatc!("{out}{'b}X {'_}Wins!\r");
                        endgame = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    raw::disable_raw_mode()?;
    print!("{}", codes::DISABLE_ALTERNATIVE_BUFFER);

    Ok(())
}
