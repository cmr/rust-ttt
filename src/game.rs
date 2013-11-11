use console_io::*;
use board::*;

mod board;
mod mock_io;
mod console_io;

struct Game {
    io: ConsoleIO,
    board: Board
}

impl Game {
    fn new(io: ConsoleIO, board: Board) -> Game {
        Game { io: io,
               board: board }
    }

    fn start(&self) {
        loop {
            //self.io.draw_board(self.board);

//            let move = self.io.get_move();
//
//            match move {
//                Some(index) => self.board = self.board.try_move(index),
//                None        => println("oops")
//            }
        }
    }

    fn single_turn(&self) {
        let move = self.io.get_move();

        match move {
            Some(index) => (),
            None        => ()
        }
    }
}

#[cfg(test)]
mod test__game {
    use super::*;
    use board::*;
    use mock_io::*;
    use console_io::*;

    #[test]
    fn draws_the_board() {
        let mut board = Board::new();
        let mock_reader_info = MockReaderInfo { str_in_stdin: ~"",
                                                read_line_call_count: 0 };

        let fake_reader = MockReader(@mock_reader_info);
        let fake_writer = MockWriter(~[]);

        let mock_io = ConsoleIO::new(fake_reader, fake_writer);
        let mut game = Game::new(mock_io, board);

        game.start();

        assert!(true);
    }
}

