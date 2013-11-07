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
        println(io.printable_board(board.clone()));

        let move = io.get_move();
        let b = board.clone();
        board = b.place(b.current_token(), move);
        if board.is_game_over() {
            println(io.printable_board(board.clone()));
            break
        }
    }
}

fn clear_screen() {
    do 1000.times {
        println("");
    }
}

