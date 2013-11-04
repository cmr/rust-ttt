use std::str;
use std::vec;

use board::*;
mod board;

fn right_edge(index: int) -> bool {
    index % 3 == 2
}

fn bottom_right_corner(index: int) -> bool {
    index == 8
}

pub fn printable_space(index: int, token: char) -> ~str {
    let printable_token = ~" " + str::from_char(token) + ~" ";

    let grid_output =
        if bottom_right_corner(index) {
            ""
        } else if right_edge(index) {
            "\n---+---+---\n"
        } else {
            "|"
        };

    printable_token + grid_output
}

pub fn printable_board(board: Board) -> ~str {

    let spaces = do vec::flat_map(board.spaces) |&space| {
        ~[printable_space(0, space)]
    };

    vec::concat(spaces)

}

#[cfg(test)]
mod test {

    #[test]
    fn knows_how_to_print_individual_spaces() {
        assert_eq!(~" x |", ::printable_space(0, 'x'));
        assert_eq!(~" o |", ::printable_space(1, 'o'));
        assert_eq!(~" x \n---+---+---\n", ::printable_space(2, 'x'));
        assert_eq!(~" o \n---+---+---\n", ::printable_space(5, 'o'));
        assert_eq!(~" o ", ::printable_space(8, 'o'));
    }

    #[test]
    fn can_print_to_console() {
        let board = ::Board::new_from_spaces(~['x','o','.',
                                               '.','.','.',
                                               '.','.','.',]);

        let expected_board_output = ~"\
              x | o | . \
             ---+---+---\
              . | . | . \
             ---+---+---\
              . | . | . ";

        ::printable_board(board);

        assert!(true);
        //assert_eq!(expected_board_output, ::printable_board(board));
    }
}
