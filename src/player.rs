use ai::*;
use board::*;
use console_input::*;

pub enum Player {
    HumanPlayer { input: ConsoleInput },
    ComputerPlayer { ai: AI }
}

impl Player {
    pub fn new_human(input: ConsoleInput) -> Player {
        HumanPlayer { input: input }
    }

    pub fn new_computer(ai: AI) -> Player {
        ComputerPlayer { ai: ai }
    }

    pub fn get_move(&self, board: Board) -> Option<int> {
        match *self {
            HumanPlayer { input: ref input } => input.get_int(),
            ComputerPlayer { ai: ref ai }    => ai.get_move(board)
        }
    }

    pub fn clone(&self) -> Player {
        match *self {
            HumanPlayer { input: ref input } => HumanPlayer { input: input.clone() },
            ComputerPlayer { ai: ref ai }    => ComputerPlayer { ai: ai.clone() }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ai::*;
    use board::*;
    use console_input::*;
    use console_reader::*;

    fn create_human_player_with_mock_input(fake_input: ~str) -> Player {
        let mock_reader = MockReader { str_in_stdin: fake_input };
        let mock_input = ConsoleInput { reader: mock_reader };
        Player::new_human(mock_input)
    }

    #[test]
    fn human_player_gets_move_from_console_input() {
        let player = create_human_player_with_mock_input(~"5\n");
        let player_with_invalid_input = create_human_player_with_mock_input(~"claws");

        let board = Board::new();

        assert_eq!(Some(5), player.get_move(board.clone()));
        assert_eq!(None, player_with_invalid_input.get_move(board.clone()));
    }

    #[test]
    fn computer_player_gets_move_from_minimax() {
        let dumb_ai = AI::new(LowestAvailable);
        let player = Player::new_computer(dumb_ai);
        let board = Board::new();

        assert_eq!(Some(0), player.get_move(board.clone()));
    }
}

