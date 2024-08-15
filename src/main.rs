use std::{borrow::Cow, env, process::ExitCode};

use args::Args;
use board::Board;
use err::{Error, Result};
use mainloop::Mainloop;
use termal::{eprintcln, gradient, printmcln};

mod append_str;
mod args;
mod board;
mod board_gui;
mod draw_buffer;
mod err;
mod mainloop;
mod slice_2d;
mod suit;
mod vec2;
mod vec2_range;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) if matches!(e, Error::Pareg(_)) => {
            eprintcln!("{'r}error: {'_}{e}");
            ExitCode::FAILURE
        }
        Err(e) => {
            _ = Mainloop::restore();
            eprintcln!("{'r}error: {'_}{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let args = Args::parse(args.iter().into())?;

    if args.help() {
        help(&args);
        return Ok(());
    }

    Mainloop::prepare()?;

    let board = Board::new(args.size(), args.win_len());
    let mut mainloop = Mainloop::new(board, args.color());
    mainloop.run()?;

    Mainloop::restore()
}

fn help(args: &Args) {
    let color = args.color();
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature: Cow<str> = if color {
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };

    printmcln!(
        color,
        "Welcome in {'i g}ttoe{'_} by {signature}{'_}
Version {v}

{'g}Usage:
  {'c}ttoe {'gr}[flags]

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Shows this help.

  {'y}-s  --size {'w}<width>x<height>{'_}
    Set the board size. Default size is largest that fits terminal.

  {'y}-w  --win  --win-length {'w}<win length>{'_}
    Set the number of same cells needed to win. The default is {'i}5{'_} or the
    larger of the board dimensions.

  {'y}--color  --colour {'w}(auto|always|never){'_}
    Determines whether color should be used.

{'g}In game controls:
  {'b}[Arrows/wasd]{'_}
    Move the selected cell (cursor).

  {'b}[Enter/Space/0]{'_}
    Play at the selected cell (cursor).

  {'b}[u]{'_}
    Undo last turn. This can undo only 1 last turn.

  {'b}[r]{'_}
    Reset. Start a new game.

  {'b}[q]{'_}
    Quit

  {'b}[Shift+Arrows/WASD]{'_}
    Move the cursor in the given direction to the first empty space after
    occupated space.

  {'b}[c]{'_}
    Move cursor to the center of the board.

  {'b}[Ctrl+Arrows/Ctrl+wasd]{'_}
    Scroll (when the board doesn't fit the terminal).

  {'b}[Alt+c]{'_}
    Toggle color.

  {'b}[C]{'_}
    Clear the last message.

  {'b}[Ctrl+c]{'_}
    Rage quit.

  {'b}[h]{'_}
    Show short help.
",
    )
}
