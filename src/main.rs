use std::process::ExitCode;

use board::Board;
use err::Result;
use mainloop::Mainloop;
use termal::eprintcln;

mod append_str;
mod board;
mod board_gui;
mod draw_buffer;
mod err;
mod mainloop;
mod suit;
mod vec2;
mod vec2_range;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            _ = Mainloop::restore();
            eprintcln!("{'r}error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    Mainloop::prepare()?;

    let mut mainloop = Mainloop::new(Board::new((15, 15)));
    mainloop.run()?;

    Mainloop::restore()
}
