use std::str;
use std::vec::*;
use std::io::*;

use board::*;
use mock_io::*;

mod mock_io;
mod board;

struct ConsoleIO {
    input: IReader
}

impl ConsoleIO {

    pub fn new(reader: IReader) -> ConsoleIO {
        ConsoleIO { input: reader }
    }

    pub fn get_move(&self) -> int {
        let input = self.input.read_line();

        let (valid_input, move) = self.valid_input(input.clone());

        if valid_input {
            move
        } else {
            self.get_move()
        }
    }

    fn valid_input(&self, s: ~str) -> (bool, int) {
        let input = match from_str::<int>(s.trim()) {
            Some(x) => x,
            None    => -1
        };

        (input >= 0 && input < 9, input)
    }

    pub fn printable_space(&self, index: int, token: char) -> ~str {
        let printable_token = " " + str::from_char(token) + " ";

        let grid_output =
            if self.is_bottom_right_corner(index) {
                ""
            } else if self.is_right_edge(index) {
                "\n---+---+---\n"
            } else {
                "|"
            };

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

    #[test]
    fn knows_how_to_print_individual_spaces() {
        let io = create_io(~"");

        assert_eq!(~" x |",               io.printable_space(0, 'x'));
        assert_eq!(~" o |",               io.printable_space(1, 'o'));
        assert_eq!(~" x \n---+---+---\n", io.printable_space(2, 'x'));
        assert_eq!(~" o \n---+---+---\n", io.printable_space(5, 'o'));
        assert_eq!(~" o ",                io.printable_space(8, 'o'));
    }

    #[test]
    fn can_print_to_console() {
        let io = create_io(~"");
        let board = ::Board::new_from_spaces(~['x','o',' ',
                                               ' ',' ',' ',
                                               ' ',' ','x' ]);

        let board_output = io.printable_board(board);

        assert!(board_output.contains(" x | o |   \n"));
        assert!(board_output.contains("---+---+---\n"));
        assert!(board_output.contains("   |   |   \n"));
        assert!(board_output.contains("---+---+---\n"));
        assert!(board_output.contains("   |   | x "));
    }

    #[test]
    fn gets_input_from_user() {
        let io = create_io(~"3\n");

        assert_eq!(3, io.get_move());
    }

    #[test]
    fn validates_input() {
        let io = create_io(~"");

        assert_eq!( (true, 0), io.valid_input(~"0\n") );
        assert_eq!( (true, 1), io.valid_input(~"1\n") );
        assert_eq!( (true, 8), io.valid_input(~"8\n") );

        assert_eq!( (false, -1), io.valid_input(~"hi\n") );
        assert_eq!( (false, -1), io.valid_input(~"\n") );
        assert_eq!( (false, 9), io.valid_input(~"9\n") );
        assert_eq!( (false, 20), io.valid_input(~"20\n") );
    }

    fn create_io(fake_input: ~str) -> ::ConsoleIO {
        let fake_reader = MockReader(fake_input);
        ::ConsoleIO::new(fake_reader)
    }
}

