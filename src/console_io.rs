use std::str;
use std::vec::*;
use std::io::*;

use board::*;
use mock_io::*;

mod mock_io;
mod board;

struct ConsoleIO {
    reader: IReader
}

#[deriving(Eq)]
pub struct ParsedInput {
    move: int,
    error_message: Option<~str>
}

impl ConsoleIO {

    pub fn new(input: IReader) -> ConsoleIO {
        ConsoleIO { reader: input }
    }

    pub fn get_move(&self, available_spaces: ~[int]) -> int {
        let input = self.reader.read_line();
        let parsed_input = self.check_valid_input(input.clone(), available_spaces.clone());

        if parsed_input.error_message == None {
            parsed_input.move
        } else {
            self.get_move(available_spaces)
        }
    }

    fn check_valid_input(&self, line: ~str, available_spaces: ~[int]) -> ParsedInput {
        let input = match from_str::<int>(line.trim()) {
                        Some(x) => x,
                        None    => -1
                    };

        match input {
            0..8 => self.check_against_available_spaces(input, available_spaces),
            _    => ParsedInput { move: -1,
                                  error_message: Some(~"Please enter a number between 0 and 8.") }
        }
    }

    fn check_against_available_spaces(&self, input: int, available_spaces: ~[int]) -> ParsedInput {
        if available_spaces.contains(&input) {
            ParsedInput { move: input, error_message: None }
        } else {
            ParsedInput { move: -1, error_message: Some(~"That space is already taken.") }
        }
    }

    fn printable_space(&self, index: int, token: char) -> ~str {
        let printable_token = " " + str::from_char(token) + " ";

        let grid_output =
            if self.is_bottom_right_corner(index) { "" }
            else if self.is_right_edge(index) { "\n---+---+---\n" }
            else { "|" };

        printable_token + grid_output
    }

    pub fn printable_board(&self, board: Board) -> ~str {

        let mut i = -1;
        let spaces = do flat_map(board.spaces) |&space| {
            i += 1;
            ~[self.printable_space(i, space)]
        };

        self.flatten(spaces)
    }

    fn is_right_edge(&self, index: int) -> bool {
        index % 3 == 2
    }

    fn is_bottom_right_corner(&self, index: int) -> bool {
        index == 8
    }

    fn flatten(&self, arr: &[~str]) -> ~str {
        let board_size = arr.len();
        let mut ans = ~"";

        for i in range(0, board_size) {
            ans = ans + arr[i];
        };

        ans
    }
}

#[cfg(test)]
mod io_test {
    use mock_io::*;

    fn create_io_with_fake_input(fake_input: ~str) -> ::ConsoleIO {
        let fake_reader = MockReader { str_in_stdin: fake_input,
                                       read_line_call_count: 0 };

        ::ConsoleIO::new(fake_reader)
    }

    #[test]
    fn generates_a_string_for_each_space() {
        let io = create_io_with_fake_input(~"");

        assert_eq!(~" x |",               io.printable_space(0, 'x'));
        assert_eq!(~" o |",               io.printable_space(1, 'o'));
        assert_eq!(~" x \n---+---+---\n", io.printable_space(2, 'x'));
        assert_eq!(~" o \n---+---+---\n", io.printable_space(5, 'o'));
        assert_eq!(~" o ",                io.printable_space(8, 'o'));
    }

    #[test]
    fn generates_a_string_for_a_board() {
        let io = create_io_with_fake_input(~"");
        let board = ::Board::new_from_spaces(~['x','o',' ',
                                               ' ',' ',' ',
                                               ' ',' ','x' ]);

        let board_output = io.printable_board(board);

        assert!(board_output.contains(" x | o |   \n" +
                                      "---+---+---\n" +
                                      "   |   |   \n" +
                                      "---+---+---\n" +
                                      "   |   | x "));
    }

    #[test]
    #[ignore]
    fn gets_input_from_user() {
        let io = create_io_with_fake_input(~"1\n2\n3\n4\n");

        assert_eq!(3, io.get_move(~[3]));
    }

    #[test]
    fn validates_input() {
        assert!(correct_error_message(~"hi", ~"Please enter a number between 0 and 8."));
        //assert!(correct_error_message(~"0", ~"That space is already taken."));

        assert!(is_valid_input(~"0\n", 0));
        assert!(is_valid_input(~"1\n", 1));
        assert!(is_valid_input(~"8\n", 8));

        assert!(is_invalid_input(~"\n"));
        assert!(is_invalid_input(~"9\n"));
        assert!(is_invalid_input(~"09\n"));
        assert!(is_invalid_input(~"20\n"));
        assert!(is_invalid_input(~"-9\n"));
        assert!(is_invalid_input(~"yo\n"));
    }

    #[test]
    fn validates_input_against_available_spaces() {
        let io = create_io_with_fake_input(~"");
        let available_spaces = ~[];

        let parsed_input = io.check_valid_input(~"0\n", available_spaces);

        assert_eq!(-1, parsed_input.move);
        assert_eq!(Some(~"That space is already taken."), parsed_input.error_message);
    }

    fn correct_error_message(input: ~str, expected_error_message: ~str) -> bool {
        let (io, available_spaces) = setup_io();
        let parsed_input = io.check_valid_input(input, available_spaces);

        match parsed_input.error_message {
            Some(error) => error == expected_error_message,
            None        => false
        }
    }

    fn is_invalid_input(input: ~str) -> bool {
        let (io, available_spaces) = setup_io();
        let expected_parsed_input = ::ParsedInput { move: -1,
                                                    error_message: Some(~"Please enter a number between 0 and 8.") };

        expected_parsed_input == io.check_valid_input(input, available_spaces)
    }

    fn is_valid_input(input: ~str, move: int) -> bool {
        let (io, available_spaces) = setup_io();
        let parsed_input = ::ParsedInput { move: move, error_message: None };

        parsed_input == io.check_valid_input(input, available_spaces)
    }

    fn setup_io() -> (::ConsoleIO, ~[int]) {
        (create_io_with_fake_input(~""), ~[0,1,2,3,4,5,6,7,8])
    }
}

