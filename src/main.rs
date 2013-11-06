use std::str;
use std::io;

use board::*;
use mock_io::*;
use console_io::*;

mod board;
mod mock_io;
mod console_io;

fn main() {
    clear_screen();

    let mut board = ::Board::new();
    let reader = RealReader(io::stdin());
    let io = ::ConsoleIO::new(reader);

    println(io.printable_board(board.clone()));

    let move = io.get_move();

    board = board.place('x', move);

    clear_screen();

    println(io.printable_board(board));
}

fn clear_screen() {
    do 1000.times {
        println("");
    }
}

