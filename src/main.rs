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

        let move = io.get_move(board.available_spaces());
        let b = board.clone();

        board = b.place(move);

        if board.is_game_over() {
            clear_screen();
            println(io.printable_board(board.clone()));
            match board.winner() {
                Some(winner) => println("\n\nWinner: " + winner.to_str()),
                None         => println("\n\nTie game!")
            }
            break
        }
    }
}

fn clear_screen() {
    // only works on vt100 terminal emulators
    println("\x1b[2J\x1b[H");
}

