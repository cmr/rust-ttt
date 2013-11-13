use board::*;
use console_input::*;
use console_output::*;
use console_reader::*;
use console_writer::*;
use player::*;

mod board;
mod console_input;
mod console_output;
mod console_reader;
mod console_writer;
mod player;

struct Game {
    output: ConsoleOutput,
    board: Board,
    player1: Player,
    player2: Player
}

impl Game {
    pub fn new(output: ConsoleOutput, board: Board, player1: Player, player2: Player) -> Game {

        Game { output: output,
               board: board,
               player1: player1,
               player2: player2
        }
    }

    pub fn next_turn(&self) -> Board {
        self.output.clear_screen();
        self.output.print_board(self.board.clone());

        let mut new_board = Board::new_from_spaces(self.board.spaces.clone());

        let move = self.player1.get_move();

        match move {
            Some(index) => new_board = self.board.try_move(index),
            None        => ()
        }

        new_board
    }
}

#[cfg(test)]
mod test__game {
    use super::*;
    use board::*;
    use player::*;
    use console_reader::*;
    use console_writer::*;
    use console_input::*;
    use console_output::*;

    #[test]
    fn can_play_a_single_turn() {
        let board = Board::new();

        let fake_reader = MockReader { str_in_stdin: ~"0" };
        let fake_input = ConsoleInput { reader: fake_reader };
        let fake_player1 = Human { input: fake_input.clone() };
        let fake_player2 = Human { input: fake_input.clone() };

        let fake_writer = @MockWriter { printed_str: ~"" };
        let fake_output = ConsoleOutput { writer: fake_writer };

        let mut game = Game::new(fake_output, board, fake_player1, fake_player2);

        game.board = game.next_turn();

        assert_eq!('x', game.board.spaces[0]);
    }
}

