use console_io::*;
use board::*;

mod board;
mod mock_io;
mod console_io;

#[deriving(Clone)]
struct Game {
    io: ConsoleIO,
    board: Board
}

impl Game {
    pub fn new(io: ConsoleIO, board: Board) -> Game {
        Game { io: io,
               board: board }
    }

    pub fn next_turn(&self) -> Game {
        let mut new_board = Board::new_from_spaces(self.board.spaces.clone());

        let move = self.io.get_move();

        match move {
            Some(index) => new_board = self.board.try_move(index),
            None        => ()
        }

        Game { io: self.io.clone(),
               board: new_board }
    }
}

#[cfg(test)]
mod test__game {
    use super::*;
    use board::*;
    use mock_io::*;
    use console_io::*;

    #[test]
    fn can_play_a_single_turn() {
        let mut board = Board::new();

        let fake_reader = MockReader { str_in_stdin: ~"0" };

        let mock_io = ConsoleIO::new(fake_reader);
        let mut game = Game::new(mock_io, board);

        game = game.next_turn();


        assert_eq!('x', game.board.spaces[0]);
    }
}

