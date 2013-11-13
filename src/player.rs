use console_reader::*;
use console_input::*;
use board::*;

mod console_reader;
mod console_input;
mod board;

pub enum Player {
    Human { input: ConsoleInput }
}

impl Player {
    pub fn new(input: ConsoleInput) -> Player {
        Human { input: input }
    }

    pub fn get_move(&self) -> Option<int> {
        match *self {
            Human { input: ref input } => input.get_move()
        }
    }
}

#[cfg(test)]
mod test__player {
    use super::*;
    use console_input::*;
    use console_reader::*;

    #[test]
    fn human_player_gets_move_from_console_input() {
        let mock_reader = MockReader { str_in_stdin: ~"5\n" };
        let mock_input = ConsoleInput { reader: mock_reader };
        let player = Player::new(mock_input);

        assert_eq!(Some(5), player.get_move());
    }
}

