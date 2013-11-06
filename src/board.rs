#[deriving(Clone)]
pub struct Board {
    spaces: ~[char]
}

impl Board {
    pub fn new() -> Board {
        Board { spaces: ~[' ',' ',' ',
                          ' ',' ',' ',
                          ' ',' ',' '] }
    }

    pub fn new_from_spaces(spaces: ~[char]) -> Board {
        Board { spaces: spaces }
    }

    pub fn place(&self, token: char, index: int) -> Board {
        let mut new_spaces = self.spaces.clone();
        new_spaces[index] = token;

        Board { spaces: new_spaces }
    }

    fn get_space(&self, index: int) -> char {
        self.spaces[index]
    }

    fn empty_spaces(&self) -> uint {
        let (empty_spaces, _) = self.spaces.clone().partition(|x: &char| *x == ' ');

        empty_spaces.len()
    }

    fn current_token(&self) -> char {
        if self.empty_spaces().is_odd() {
            'x'
        } else {
            'o'
        }
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
                                               ' ',' ',' ',]);

        assert_eq!('x', board.get_space(0));
        assert_eq!('o', board.get_space(1));
    }

    #[test]
    fn can_place_a_token() {
        let mut board = ::Board::new();

        board = board.place('x', 0);
        board = board.place('o', 1);

        assert_eq!('x', board.get_space(0));
        assert_eq!('o', board.get_space(1));
        assert_eq!(' ', board.get_space(2));
        assert_eq!(' ', board.get_space(2));
    }

    #[test]
    fn knows_the_current_token() {
        let mut board = ::Board::new();
        assert_eq!('x', board.current_token());

        board = board.place('x', 0);
        assert_eq!('o', board.current_token());
    }
}

