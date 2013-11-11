use std::str;
use std::io;

use board::*;
use game::*;
use mock_io::*;
use console_io::*;

mod board;
mod game;
mod mock_io;
mod console_io;

fn main() {
    let mut board = Board::new();
    let reader = RealReader(io::stdin());
    let io = ConsoleIO::new(reader);

    let mut game = Game::new(io, board.clone());

    loop {
        game = game.next_turn();

        if game.board.is_game_over() {
            clear_screen();
            print_board(game.clone());

            break
        }
    }
}

fn print_board(game: Game) {
    println(game.io.printable_board(game.board.clone()));
}

fn clear_screen() {
    // only works on vt100 terminal emulators
    println("\x1b[2J\x1b[H");
}

