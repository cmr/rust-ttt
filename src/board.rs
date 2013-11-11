use std::iter::Repeat;
use std::num::sqrt;

#[deriving(Clone, Eq)]
struct Board {
    spaces: ~[char],
    error_message: Option<~str>
}

impl Board {
    pub fn new() -> Board {
        let empty_spaces: ~[char] = Repeat::new(' ').take(9).collect::<~[char]>();

        Board { spaces: empty_spaces,
                error_message: None }
    }

    pub fn new_from_spaces(spaces: ~[char]) -> Board {
        Board { spaces: spaces,
                error_message: None }
    }

    pub fn place(&self, index: int) -> Board {
        let mut new_spaces = self.spaces.clone();

        if new_spaces[index] == ' ' {
            new_spaces[index] = self.current_token();
        }

        Board { spaces: new_spaces,
                error_message: None }
    }

    fn try_move(&self, index: int) -> Board {
        let error_message = self.get_error_message(index);

        match error_message {
            Some(*) => Board { spaces: self.spaces.clone(),
                               error_message: error_message },
            None => Board { spaces: self.place(index).spaces,
                            error_message: error_message }
        }

    }

    fn get_error_message(&self, index: int) -> Option<~str> {
        match index {
            0..8 => self.check_against_available_moves(index),
            _    => Some(~"Please choose a number from 0 to 8.")
        }
    }

    fn check_against_available_moves(&self, index: int) -> Option<~str> {
        if self.available_spaces().contains(@index) {
            None
        } else {
            Some(~"That space is already taken.")
        }
    }

    fn empty_spaces(&self) -> uint {
        let mut empty_spaces = self.spaces.clone().to_owned();
        empty_spaces.retain( |x: &char| *x == ' ' );

        empty_spaces.len()
    }

    pub fn current_token(&self) -> char {
        if self.empty_spaces().is_odd() {
            'x'
        } else {
            'o'
        }
    }

    fn transpose(&self) -> Board {
        let orig_spaces = self.spaces.clone();
        let mut new_spaces = ~[];
        let mut i: uint = 0;
        let dim = self.dimension();
        let size = dim * dim;

        do (size - 1).times {
            new_spaces.push(orig_spaces[(i * dim) % (size - 1)]);
            i += 1;
        }

        new_spaces.push(orig_spaces[8]);

        Board { spaces: new_spaces,
                error_message: None }
    }

    fn is_winning(&self, line: &[char]) -> bool {
        self.is_all_same_token(line.to_owned())
    }

    fn dimension(&self) -> uint {
        let size = self.spaces.len() as float;
        sqrt(size) as uint
    }

    fn has_backslash_diagonal_winner(&self) -> bool{
        let mut i = 0;
        let dim = self.dimension();
        let mut diagonal_tokens: ~[char] = ~[];

        do dim.times {
            diagonal_tokens.push(self.spaces[(dim + 1) * i]);
            i += 1;
        }

        self.is_all_same_token(diagonal_tokens)
    }

    fn has_slash_diagonal_winner(&self) -> bool{
        let mut i = 1;
        let dim = self.dimension();
        let mut diagonal_tokens: ~[char] = ~[];

        do dim.times {
            diagonal_tokens.push(self.spaces[(dim - 1) * i]);
            i += 1;
        }

        self.is_all_same_token(diagonal_tokens)
    }

    fn is_all_same_token(&self, tokens: ~[char]) -> bool {
        let first_token = tokens[0];

        tokens.iter().all( |x: &char| *x == first_token) && first_token != ' '
    }

    pub fn winner(&self) -> Option<char> {
        let mut rows = self.spaces.chunk_iter(3);
        let transposed = self.transpose();
        let mut columns = transposed.spaces.chunk_iter(3);

        for row in rows {
            if self.is_winning(row) {
                return Some(row[0]);
            }
        }

        for column in columns {
            if self.is_winning(column) {
                return Some(column[0]);
            }
        }


        if self.has_slash_diagonal_winner() && self.spaces[self.dimension() - 1] != ' ' {
            return Some(self.spaces[self.dimension() - 1]);
        }

        if self.has_backslash_diagonal_winner() && self.spaces[0] != ' ' {
            return Some(self.spaces[0]);
        }

        None
    }

    fn somebody_won(&self) -> bool {
        !(self.winner() == None)
    }

    fn board_is_full(&self) -> bool {
        self.empty_spaces() == 0
    }

    pub fn is_game_over(&self) -> bool {
        self.somebody_won() || self.board_is_full()
    }

    pub fn available_spaces(&self) -> ~[int] {
        let mut available_spaces: ~[int] = ~[];
        let mut i = 0;
        let size = self.spaces.len();

        do size.times {
            if self.spaces[i] == ' ' {
                available_spaces.push(i);
            }

            i += 1;
        }

        available_spaces
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn has_9_spaces() {
        let board = ::Board::new();

        assert_eq!(9, board.spaces.len());
    }

    #[test]
    fn can_create_empty_board() {
        let board = ::Board::new();

        for space in board.spaces.iter() {
            assert_eq!(' ', *space);
        }
    }

    #[test]
    fn can_create_from_another_board() {
        let board = ::Board::new_from_spaces(~['x','o',' ',
                                               ' ',' ',' ',
                                               ' ',' ',' ' ]);

        assert_eq!('x', board.spaces[0]);
        assert_eq!('o', board.spaces[1]);
    }

    #[test]
    fn can_place_a_token() {
        let mut board = ::Board::new();

        board = board.place(0);
        board = board.place(1);

        assert_eq!('x', board.spaces[0]);
        assert_eq!('o', board.spaces[1]);
        assert_eq!(' ', board.spaces[2]);
    }

    #[test]
    fn can_only_place_a_token_in_an_empty_space() {
        let mut board = ::Board::new();

        board = board.place(0);
        board = board.place(0);

        assert_eq!('x', board.spaces[0]);
        assert_eq!('o', board.current_token());
    }

    #[test]
    fn knows_the_current_token() {
        let mut board = ::Board::new();
        assert_eq!('x', board.current_token());

        board = board.place(0);
        assert_eq!('o', board.current_token());
    }

    #[test]
    fn knows_someone_wins_when_they_have_a_row() {
        let x_wins_boards = [::Board::new_from_spaces(~['x','x','x',
                                                        'o','o',' ',
                                                        ' ',' ',' ' ]),

                             ::Board::new_from_spaces(~['o','o',' ',
                                                        'x','x','x',
                                                        ' ',' ',' ' ]),

                             ::Board::new_from_spaces(~['o','o',' ',
                                                        ' ',' ',' ',
                                                        'x','x','x' ])];

        for board in x_wins_boards.iter() { assert_eq!(Some('x'), board.winner()) }

        let o_wins_board = ::Board::new_from_spaces(~['o','o','o',
                                                      'x','x',' ',
                                                      'x',' ',' ' ]);

        assert_eq!(Some('o'), o_wins_board.winner())
    }

    #[test]
    fn can_transpose_the_spaces() {
        let board = ::Board::new_from_spaces(~['0','1','2',
                                               '3','4','5',
                                               '6','7','8' ]);

        let transposed_board = ::Board::new_from_spaces(~['0','3','6',
                                                          '1','4','7',
                                                          '2','5','8' ]);

        assert_eq!(transposed_board, board.transpose());
    }

    #[test]
    fn knows_someone_wins_when_they_have_a_column() {
        let x_wins_boards = [::Board::new_from_spaces(~['x','o',' ',
                                                        'x','o',' ',
                                                        'x',' ',' ' ]),

                             ::Board::new_from_spaces(~['o','x',' ',
                                                        'o','x',' ',
                                                        ' ','x',' ' ]),

                             ::Board::new_from_spaces(~['o','o','x',
                                                        ' ',' ','x',
                                                        ' ',' ','x' ])];

        for board in x_wins_boards.iter() { assert_eq!(Some('x'), board.winner()) }

        let o_wins_board = ::Board::new_from_spaces(~['o','x','x',
                                                      'o','x',' ',
                                                      'o',' ',' ' ]);

        assert_eq!(Some('o'), o_wins_board.winner())
    }

    #[test]
    fn knows_someone_wins_when_they_have_a_diagonal() {
        let x_wins_boards = [::Board::new_from_spaces(~['x','o',' ',
                                                        'o','x',' ',
                                                        ' ',' ','x' ]),

                             ::Board::new_from_spaces(~['o',' ','x',
                                                        'o','x',' ',
                                                        'x',' ',' ' ])];

        for board in x_wins_boards.iter() { assert_eq!(Some('x'), board.winner()) }

        let o_wins_board = ::Board::new_from_spaces(~['o','x','x',
                                                      ' ','o',' ',
                                                      ' ','x','o' ]);

        assert_eq!(Some('o'), o_wins_board.winner())
    }

    #[test]
    fn knows_when_nobody_wins() {
        let unfinished_boards = [::Board::new(),

                                 ::Board::new_from_spaces(~['o',' ',' ',
                                                            'o','x',' ',
                                                            'x',' ',' ' ]),

                                 ::Board::new_from_spaces(~['x','x','o',
                                                            'o','o','x',
                                                            'x','o','x' ])];

        for board in unfinished_boards.iter() { assert_eq!(None, board.winner()) }
    }

    #[test]
    fn knows_when_the_game_is_over() {
        let game_over_boards = [::Board::new_from_spaces(~['o','o','o',
                                                           'x','x',' ',
                                                           ' ','x',' ' ]),

                                ::Board::new_from_spaces(~['o',' ','x',
                                                           'o','x',' ',
                                                           'x',' ',' ' ]),

                                ::Board::new_from_spaces(~['x','x','o',
                                                           'o','o','x',
                                                           'x','o','x' ])];

        for board in game_over_boards.iter() { assert!(board.is_game_over()) }
    }

    #[test]
    fn knows_when_the_game_is_not_over() {
        let game_over_boards = [::Board::new(),

                                ::Board::new_from_spaces(~['o',' ',' ',
                                                           'o','x',' ',
                                                           'x',' ',' ' ]),

                                ::Board::new_from_spaces(~['x','x','o',
                                                           'o',' ','x',
                                                           'x','o','x' ])];

        for board in game_over_boards.iter() { assert!(!board.is_game_over()) }
    }

    #[test]
    fn knows_the_available_spaces() {
        let empty_board = ::Board::new();

        let board = ::Board::new_from_spaces(~['x','o',' ',
                                               ' ','x',' ',
                                               ' ','o',' ' ]);

        assert_eq!(~[0,1,2,3,4,5,6,7,8], empty_board.available_spaces());
        assert_eq!(~[2,3,5,6,8], board.available_spaces());
    }

    #[test]
    fn starts_with_no_error_message() {
        let board = ::Board::new();

        assert_eq!(None, board.error_message);
    }

    #[test]
    fn sets_an_error_message_for_invalid_moves() {
        let board = ::Board::new_from_spaces(~['x','o',' ',
                                               ' ','x',' ',
                                               ' ','o',' ' ]);

        let space_already_taken_board = board.try_move(0);
        let invalid_space_board = board.try_move(1000);

        assert_eq!(Some(~"That space is already taken."), space_already_taken_board.error_message);
        assert_eq!(Some(~"Please choose a number from 0 to 8."), invalid_space_board.error_message);
    }

    #[test]
    fn try_move_can_place_a_valid_token() {
        let mut board = ::Board::new_from_spaces(~[' ','x',' ',
                                                   ' ','x',' ',
                                                   ' ','o',' ' ]);

        let board = board.try_move(0);

        assert_eq!('o', board.spaces[0]);
    }
}

