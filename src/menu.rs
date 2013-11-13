use console_input::*;

mod console_input;
mod console_reader;

condition! {
    pub quit_choice: ~str -> int;
}

pub struct Menu {
    input: ConsoleInput
}

impl Menu {
    pub fn new(input: ConsoleInput) -> Menu {
        Menu { input: input }
    }

    pub fn get_menu_choice(&self) -> Option<int> {
        let input = self.input.get_int();

        match input {
            Some(choice) => self.validate_choice(choice),
            None         => None
        }
    }

    fn validate_choice(&self, choice: int) -> Option<int> {
        match choice {
            0..4 => Some(choice),
            5    => Some(quit_choice::cond.raise(~"Exit successful.")),
            _    => None
        }
    }
}

#[cfg(test)]
mod test__menu {
    use super::*;
    use console_input::*;
    use console_reader::*;

    fn create_menu_with_mock(fake_input: ~str) -> Menu {
        let mock_reader = MockReader { str_in_stdin: fake_input };
        let mock_input = ConsoleInput { reader: mock_reader };

        Menu::new(mock_input)
    }

    #[test]
    fn uses_console_reader_to_get_menu_choice() {
        let menu = create_menu_with_mock(~"1\n");
        let menu_with_invalid_choice = create_menu_with_mock(~"6\n");

        assert_eq!(Some(1), menu.get_menu_choice());
        assert_eq!(None, menu_with_invalid_choice.get_menu_choice());
    }

    #[test]
    fn can_quit_from_menu() {
        do quit_choice::cond.trap(|_| -1).inside {
            let menu = create_menu_with_mock(~"5\n");

            let choice = menu.get_menu_choice();
            assert_eq!(Some(-1), choice);
        }
    }
}

