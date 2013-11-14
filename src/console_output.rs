use std::str;
use std::vec::*;

use board::*;
use console_writer::*;

struct ConsoleOutput {
    writer: @ConsoleWriter
}

impl ConsoleOutput {
    pub fn new(writer: @ConsoleWriter) -> ConsoleOutput {
        ConsoleOutput { writer: writer }
    }

    pub fn print_board(&self, board: Board) -> @ConsoleWriter {
        let printable_board = self.printable_board(board.clone());

        self.writer.println(printable_board)
    }

    pub fn printable_board(&self, board: Board) -> ~str {
        let mut i = -1;
        let spaces = do flat_map(board.spaces) |&space| {
            i += 1;
            ~[self.printable_space(i, space)]
        };

        self.flatten(spaces) + self.printable_error_message(board.flash_message)
    }

    fn printable_space(&self, index: int, token: char) -> ~str {
        let printable_token = " " + str::from_char(token) + " ";

        let grid_output =
            if self.is_bottom_right_corner(index) { "" }
            else if self.is_right_edge(index)     { "\n---+---+---\n" }
            else                                  { "|" };

        printable_token + grid_output
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

    pub fn clear_screen(&self) {
        // only works on vt100 terminal emulators
        self.writer.println(~"\x1b[2J\x1b[H");
    }

    pub fn print_menu(&self) -> @ConsoleWriter {
        let menu_str = ~"Please select the game type:\n\n" +
                        " 1) Human vs. Human\n" +
                        " 2) Human vs. Computer\n" +
                        " 3) Computer vs. Human\n" +
                        " 4) Computer vs. Computer\n" +
                        " 5) Quit\n\n";

        self.writer.println(menu_str)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use board::*;
    use console_writer::*;

    fn output_with_fake_writer() -> ConsoleOutput {
        let mock_writer = MockWriter { printed_str: ~"" };

        ConsoleOutput::new(@mock_writer)
    }

    #[test]
    fn generates_a_string_for_each_space() {
        let output = output_with_fake_writer();

        assert_eq!(~" x |",               output.printable_space(0, 'x'));
        assert_eq!(~" o |",               output.printable_space(1, 'o'));
        assert_eq!(~" x \n---+---+---\n", output.printable_space(2, 'x'));
        assert_eq!(~" o \n---+---+---\n", output.printable_space(5, 'o'));
        assert_eq!(~" o ",                output.printable_space(8, 'o'));
    }

    #[test]
    fn can_print_an_error_message() {
        let output = output_with_fake_writer();

        assert_eq!(~"\n\nfake error message", output.printable_error_message(Some(~"fake error message")));
    }

    #[test]
    fn prints_error_message_with_board() {
        let output = output_with_fake_writer();

        let mut board = Board::new_from_spaces(~['x','o',' ',
                                                 ' ',' ',' ',
                                                 ' ',' ','x' ]);

        board = board.try_move(0);

        assert!(output.printable_board(board).contains("That space is already taken."));
    }

    #[test]
    fn can_print_a_board() {
        let mut output = output_with_fake_writer();

        let board = Board::new_from_spaces(~['x','o',' ',
                                             ' ',' ',' ',
                                             ' ',' ','x' ]);

        output.writer = output.print_board(board);
        let printed_str = output.writer.get_printed_str();

        assert!(printed_str.contains(" x | o |   \n" +
                                     "---+---+---\n" +
                                     "   |   |   \n" +
                                     "---+---+---\n" +
                                     "   |   | x "));
    }

    #[test]
    fn can_print_the_menu() {
        let mut output = output_with_fake_writer();

        output.writer = output.print_menu();
        let printed_str = output.writer.get_printed_str();

        assert!(printed_str.contains("Please select the game type:\n\n" +
                                     " 1) Human vs. Human\n" +
                                     " 2) Human vs. Computer\n" +
                                     " 3) Computer vs. Human\n" +
                                     " 4) Computer vs. Computer\n" +
                                     " 5) Quit\n\n"));
    }
}

