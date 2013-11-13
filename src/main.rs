use std::str;
use std::io;
use board::*;
use console_input::*;
use console_output::*;
use console_reader::*;
use console_writer::*;
use game::*;
use player::*;

mod board;
mod console_input;
mod console_output;
mod console_reader;
mod console_writer;
mod game;
mod player;

fn main() {
    let mut board = Board::new();
    let reader = RealReader(io::stdin());
    let writer = RealWriter;
    let input = ConsoleInput::new(reader);
    let output = ConsoleOutput::new(@writer);
    let player1 = Human { input: input.clone() };
    let player2 = Human { input: input.clone() };

    let mut game = Game::new(output, board, player1, player2);

    loop {
        game.output.clear_screen();
        game.output.print_board(game.board.clone());

        game.board = game.next_turn().clone();

        if game.board.is_game_over() {
            game.output.clear_screen();
            game.output.print_board(game.board.clone());

            break
        }
    }
}

