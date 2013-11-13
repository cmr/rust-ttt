use std::vec::*;
use board::*;

mod board;

#[deriving(Clone)]
pub enum Strategy {
    Minimax,
    LowestAvailable
}

pub struct AI {
    strategy: Strategy
}

impl AI {
    pub fn new(strategy: Strategy) -> AI {
        AI { strategy: strategy }
    }

    pub fn get_move(&self, spaces: ~[char]) -> Option<int> {
        ::std::rt::io::timer::sleep(1000); // simulate thinking

        match (*self).strategy {
            LowestAvailable => self.get_lowest_available_index(spaces),
            Minimax         => self.minimax(spaces)
        }
    }

    fn get_lowest_available_index(&self, spaces: ~[char]) -> Option<int> {
        let empty_spaces = spaces.clone().to_owned();
        let position = empty_spaces.iter().position( |x: &char| *x == ' ' );

        match position {
            Some(index) => Some(index as int),
            None        => None
        }
    }

    pub fn clone(&self) -> AI {
        AI { strategy: self.strategy.clone() }
    }

    fn minimax(&self, spaces: ~[char]) -> Option<int> {
        Some(-1)
    }

//    pub fn score_spaces(&self, board: Board) -> ~[int] {
//        let mut i = -1;
//
//        do flat_map(board.spaces.clone()) |_| {
//            i += 1;
//            ~[self.score_move(i, board.clone())]
//        }
//    }

    pub fn score_move(&self, index: int, board: Board) -> Option<int> {
        let current_token = board.current_token();

        if board.available_spaces().contains(&index) { // if space is empty
            let new_board = board.place(index);

            if new_board.is_game_over() { // if new move finishes the game
                // return Option<int> score for index
                self.score_finished_board(new_board.clone())
            } else {
                // score rest of board
                None
            }

        } else { // if space is already taken
            None
        }
//            let mut new_board = board.place(index);
//            println("\n\n" + new_board.spaces.to_str() + "\n");
//
//            if new_board.is_game_over() {
//                match new_board.winner() {
//                    Some(winner) => self.score_game_over_board(winner, current_token),
//                    None         => 0
//                }
//            } else {
//                //-1
//                let scores = self.score_spaces(new_board);
//                let ind = self.index_of_max_score(scores.clone());
//
//                scores[ind]
//            }
//        }
    }

    pub fn score_finished_board(&self, board: Board) -> Option<int> {
        match board.winner() {
            Some(winner) => Some(1),
            None         => Some(0)
        }
    }

    fn score_game_over_board(&self, winner: char, current_token: char) -> int {
        if winner == current_token {
            -3
        } else {
            3
        }
    }

    fn index_of_max_score(&self, scores: ~[int]) -> int {
        let max_value = scores.iter().max().unwrap();

        scores.iter().position( |x: &int| *x == *max_value ).unwrap() as int
    }
}

#[cfg(test)]
mod test__minimax {
    use super::*;
    use board::*;

    #[test]
    fn can_score_finished_boards() {
        let ai = AI::new(Minimax);
        let x_wins_board = Board::new_from_spaces(~['o','x','o',
                                                    'o','x','x',
                                                    ' ','x',' ' ]);

        let x_wins_score = match ai.score_finished_board(x_wins_board) {
            Some(score) => score,
            None        => fail!()
        };


        let tie_game_board = Board::new_from_spaces(~['o','x','o',
                                                      'x','o','x',
                                                      'x','o','x' ]);

        let tie_game_score = match ai.score_finished_board(tie_game_board) {
            Some(score) => score,
            None        => fail!()
        };

        assert!(x_wins_score > 0);
        assert!(tie_game_score == 0);
    }

    #[test]
    fn scores_unavailable_spaces_as_None() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             ' ',' ',' ' ]);

        assert_eq!(None, ai.score_move(0, board.clone()));
    }

    #[test]
    fn scores_tying_moves_as_0() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             'x','o',' ' ]);

        let tying_score = match ai.score_move(8, board.clone()) {
            Some(score) => score,
            None        => fail!()
        };

        //println("\n\ntying_score:" + tying_score.to_str() + "\n");
        assert_eq!(0, tying_score);
    }

    #[test]
    fn scores_winning_moves_positively() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             ' ',' ',' ' ]);

        let winning_score = match ai.score_move(7, board.clone()) {
            Some(score) => score,
            None        => fail!()
        };

        //println("\n\nwinning: " + winning_score.to_str() + "\n");
        assert!(winning_score > 0);
    }

//    #[test]
//    fn scores_losing_moves_negatively() {
//        let ai = AI::new(Minimax);
//        let board = Board::new_from_spaces(~['o','x','o',
//                                             'o','x','x',
//                                             ' ',' ',' ' ]);
//
//        let winning_score = match ai.score_move(8, board.clone()) {
//            Some(score) => score,
//            None        => fail!()
//        };
//
//        println("\n\nscores[8]\n" + winning_score.to_str() + "\n");
//        assert!(winning_score < 0);
//    }
}
