use std::str;

use board::*;
mod board;

fn main() {
    println("in main...");

    let board = ::Board::new_from_spaces(~['o']);

    println(str::from_char(board.spaces[0]));
    println("done");
}
