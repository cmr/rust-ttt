use std::io;
use board::*;
use console_input::*;
use console_output::*;
use console_reader::*;
use console_writer::*;
use game::*;
use menu::*;
use player::*;

mod board;
mod console_input;
mod console_output;
mod console_reader;
mod console_writer;
mod game;
mod menu;
mod player;

fn main() {
    loop {
        let (menu, board, input, output) = setup();
        let (player1, player2) = setup_players(menu, input.clone(), output);

        let mut game = Game::new(output, board, player1, player2);

        loop {
            game.board = game.next_turn().clone();

            if game.board.is_game_over() {
                game.output.clear_screen();
                game.output.print_board(game.board.clone());
                std::rt::io::timer::sleep(3000);

                break
            }
        }
    }
}

fn setup() -> (Menu, Board, ConsoleInput, ConsoleOutput) {
    let board = Board::new();

    let reader = RealReader(io::stdin());
    let input = ConsoleInput::new(reader.clone());

    let menu = Menu::new(input.clone());

    let output = ConsoleOutput::new(@RealWriter);

    (menu, board, input.clone(), output)
}

fn setup_players(menu: Menu, input: ConsoleInput, output: ConsoleOutput) -> (Player, Player) {
    output.clear_screen();
    output.print_menu();

    match menu.get_menu_choice() {
        Some(choice) => match_choice(choice, input),
        None         => setup_players(menu, input, output)
    }
}

fn match_choice(choice: int, input: ConsoleInput) -> (Player, Player) {
    match choice {
        1 => (Human { input: input.clone() },
              Human { input: input.clone() }),
        2 => (Human { input: input.clone() },
              Human { input: input.clone() }),
        3 => (Human { input: input.clone() },
              Human { input: input.clone() }),
        4 => (Human { input: input.clone() },
              Human { input: input.clone() }),
        _ => (Human { input: input.clone() },
              Human { input: input.clone() })
    }
}

