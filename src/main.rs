use std::str;
use std::io;

use board::*;
use mock_io::*;
use console_io::*;

mod board;
mod mock_io;
mod console_io;

fn main() {
    let mut board = ::Board::new();
    let reader = RealReader(io::stdin());
    let io = ::ConsoleIO::new(reader);

    loop {
        clear_screen();
        print_board();

        let move = io.get_move();
        board = board.place(board.current_token(), move);
    }
}

fn print_board(board: Board) {
    println(io.printable_board(board.clone()));
}

fn clear_screen() {
    do 1000.times {
        println("");
    }
}

