use board::Board;
use spot::Suit;
use termal::codes;

mod board;
mod spot;
mod board_gui;
mod err;
mod vec2;

fn main() {
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

    let mut board = Board::new(5, 5);
    board[(2, 2)] = Suit::Circle;
    board[(2, 3)] = Suit::Cross;
    let mut out = String::new();
    board.draw(&mut out, |s, x, y| *s += &termal::move_to!(x + 1, y + 1));
    println!("{}{}", codes::ERASE_SCREEN, out);
}
