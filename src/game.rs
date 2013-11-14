use board::*;
use console_output::*;
use player::*;

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
        let spaces = self.board.spaces.clone();

        self.output.print_board(self.board.clone());

        let mut new_board = Board::new_from_spaces(spaces.clone());

        let move = if self.board.current_token() == 'x' {
            self.player1.get_move(new_board.clone())
        } else {
            self.player2.get_move(new_board.clone())
        };

        match move {
            Some(index) => new_board = self.board.try_move(index),
            None        => ()
        }

        new_board
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use board::*;
    use player::*;
    use console_reader::*;
    use console_writer::*;
    use console_input::*;
    use console_output::*;

    fn create_human_with_input(input: ~str) -> Player {
        let fake_reader = MockReader { str_in_stdin: input };
        let fake_input = ConsoleInput { reader: fake_reader };

        HumanPlayer { input: fake_input.clone() }
    }

    fn create_fake_output() -> ConsoleOutput {
        let fake_writer = @MockWriter { printed_str: ~"" };
        ConsoleOutput { writer: fake_writer }
    }

    #[test]
    fn can_play_single_turns() {
        let board = Board::new();
        let fake_player1 = create_human_with_input(~"0");
        let fake_player2 = create_human_with_input(~"1");
        let fake_output = create_fake_output();

        let mut game = Game::new(fake_output, board, fake_player1, fake_player2);

        game.board = game.next_turn();
        assert_eq!('x', game.board.spaces[0]);
        assert_eq!(' ', game.board.spaces[1]);

        game.board = game.next_turn();
        assert_eq!('o', game.board.spaces[1]);
    }
}

