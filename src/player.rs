use console_input::*;

mod console_input;
mod console_reader;

pub enum Player {
    Human { input: ConsoleInput }
}

impl Player {
    pub fn new(input: ConsoleInput) -> Player {
        Human { input: input }
    }

    pub fn get_move(&self) -> Option<int> {
        match *self {
            Human { input: ref input } => input.get_int()
        }
    }

    pub fn clone(&self) -> Player {
        match *self {
            Human { input: ref input } => Human { input: input.clone() }
        }
    }
}

#[cfg(test)]
mod test__player {
    use super::*;
    use console_input::*;
    use console_reader::*;

    fn create_player_with_mock_input(fake_input: ~str) -> Player {
        let mock_reader = MockReader { str_in_stdin: fake_input };
        let mock_input = ConsoleInput { reader: mock_reader };
        Player::new(mock_input)
    }

    #[test]
    fn human_player_gets_move_from_console_input() {
        let player = create_player_with_mock_input(~"5\n");
        let player_with_invalid_input = create_player_with_mock_input(~"claws");

        assert_eq!(Some(5), player.get_move());
        assert_eq!(None, player_with_invalid_input.get_move());
    }
}

