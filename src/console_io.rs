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

    pub fn get_move(&self) -> Option<int> {
        let input = self.reader.read_line();

        from_str::<int>(input.trim())
    }

    pub fn print_board(&self, board: Board) {
        self.clear_screen();
        println(self.printable_board(board));
    }

    fn clear_screen(&self) {
        // only works on vt100 terminal emulators
        println("\x1b[2J\x1b[H");
    }

    fn printable_space(&self, index: int, token: char) -> ~str {
        let printable_token = " " + str::from_char(token) + " ";

        let grid_output =
            if self.is_bottom_right_corner(index) { "" }
            else if self.is_right_edge(index)     { "\n---+---+---\n" }
            else                                  { "|" };

        printable_token + grid_output
    }

    pub fn printable_board(&self, board: Board) -> ~str {
        let mut i = -1;
        let spaces = do flat_map(board.spaces) |&space| {
            i += 1;
            ~[self.printable_space(i, space)]
        };

        self.flatten(spaces) + self.printable_error_message(board.flash_message)
    }

    fn printable_error_message(&self, error: Option<~str>) -> ~str {
        match error {
            Some(message) => ~"\n\n" + message,
            None          => ~""
        }
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

    pub fn clone(&self) -> ConsoleIO {
        ConsoleIO::new(self.reader.clone())
    }
}

#[cfg(test)]
mod test__io {
    use super::*;
    use board::*;
    use mock_io::*;

    fn create_io_with_mocks(fake_input: ~str) -> ConsoleIO {
        let fake_reader = MockReader { str_in_stdin: fake_input };
        ConsoleIO::new(fake_reader)
    }

    #[test]
    fn generates_a_string_for_each_space() {
        let io = create_io_with_mocks(~"");

        assert_eq!(~" x |",               io.printable_space(0, 'x'));
        assert_eq!(~" o |",               io.printable_space(1, 'o'));
        assert_eq!(~" x \n---+---+---\n", io.printable_space(2, 'x'));
        assert_eq!(~" o \n---+---+---\n", io.printable_space(5, 'o'));
        assert_eq!(~" o ",                io.printable_space(8, 'o'));
    }

    #[test]
    fn generates_a_string_for_a_board() {
        let io = create_io_with_mocks(~"");
        let board = Board::new_from_spaces(~['x','o',' ',
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
    fn gets_input_from_user() {
        let io = create_io_with_mocks(~"  1  \n");
        assert_eq!(Some(1), io.get_move());

        let io_with_invalid_input = create_io_with_mocks(~"wazzup\n");
        assert_eq!(None, io_with_invalid_input.get_move());
    }

    #[test]
    fn can_print_an_error_message() {
        let io = create_io_with_mocks(~"");

        assert_eq!(~"\n\nfake error message", io.printable_error_message(Some(~"fake error message")));
    }

    #[test]
    fn prints_error_message_with_board() {
        let io = create_io_with_mocks(~"");
        let mut board = Board::new_from_spaces(~['x','o',' ',
                                                 ' ',' ',' ',
                                                 ' ',' ','x' ]);

        board = board.try_move(0);

        assert!(io.printable_board(board).contains("That space is already taken."));
    }
}

