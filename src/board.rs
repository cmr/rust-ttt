use std::iter::Repeat;

#[deriving(Clone, Eq)]
struct Board {
    spaces: ~[char]
}

impl Board {
    pub fn new() -> Board {
        let empty_spaces: ~[char] = Repeat::new(' ').take(9).collect::<~[char]>();

        Board { spaces: empty_spaces }
    }

    pub fn new_from_spaces(spaces: ~[char]) -> Board {
        Board { spaces: spaces }
    }

    pub fn place(&self, token: char, index: int) -> Board {
        let mut new_spaces = self.spaces.clone();
        new_spaces[index] = token;

        Board { spaces: new_spaces }
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
        let dim = 3;
        let size = dim * dim;

        do (size - 1).times {
            new_spaces.push(orig_spaces[(i * dim) % (size - 1)]);
            i += 1;
        }

        new_spaces.push(orig_spaces[8]);

        Board { spaces: new_spaces }
    }

    fn is_winning(&self, row: &[char]) -> bool {
        let mut owned_row = row.to_owned();
        let first_token = owned_row.shift();

        if !(first_token == ' ') {
            owned_row.retain(|x: &char| *x == first_token);
            let tokens_in_row = owned_row.len() + 1;

            match tokens_in_row {
                3 => return true,
                _ => ()
            }
        }

        return false;
    }

    fn dimension(&self) -> uint {
        let size = self.spaces.len() as float;
        std::num::sqrt(size) as uint
    }

    fn has_backslash_diagonal_winner(&self) -> bool{
        let mut i = 0;
        let dim = self.dimension();
        let mut diagonal_tokens: ~[char] = ~[];
        do dim.times {
            diagonal_tokens.push(self.spaces[(dim + 1) * i]);
            i += 1;
        }

        diagonal_tokens[0] == diagonal_tokens[1] && diagonal_tokens[0] == diagonal_tokens[2]
    }

    fn has_slash_diagonal_winner(&self) -> bool{
        let mut i = 1;
        let dim = self.dimension();
        let mut diagonal_tokens: ~[char] = ~[];
        do dim.times {
            diagonal_tokens.push(self.spaces[(dim - 1) * i]);
            i += 1;
        }

        diagonal_tokens[0] == diagonal_tokens[1] && diagonal_tokens[0] == diagonal_tokens[2]
    }

    fn winner(&self) -> Option<char> {
        let mut rows = self.spaces.chunk_iter(3);
        let transposed = self.transpose();
        let mut columns = transposed.spaces.chunk_iter(3);

        for row in rows {
            if self.is_winning(row) && row[0] != ' ' {
                return Some(row[0]);
            }
        }

        for column in columns {
            if self.is_winning(column) && column[0] != ' ' {
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

    fn is_game_over(&self) -> bool {
        !(self.winner() == None) || self.empty_spaces() == 0
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

        board = board.place('x', 0);
        board = board.place('o', 1);

        assert_eq!('x', board.spaces[0]);
        assert_eq!('o', board.spaces[1]);
        assert_eq!(' ', board.spaces[2]);
    }

    #[test]
    fn knows_the_current_token() {
        let mut board = ::Board::new();
        assert_eq!('x', board.current_token());

        board = board.place('x', 0);
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
        let unfinished_boards = [::Board::new_from_spaces(~[' ',' ',' ',
                                                            ' ',' ',' ',
                                                            ' ',' ',' ' ]),

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
        let game_over_boards = [::Board::new_from_spaces(~[' ',' ',' ',
                                                           ' ',' ',' ',
                                                           ' ',' ',' ' ]),

                                ::Board::new_from_spaces(~['o',' ',' ',
                                                           'o','x',' ',
                                                           'x',' ',' ' ]),

                                ::Board::new_from_spaces(~['x','x','o',
                                                           'o',' ','x',
                                                           'x','o','x' ])];

        for board in game_over_boards.iter() { assert!(!board.is_game_over()) }
    }
}

