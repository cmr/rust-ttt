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

    pub fn get_move(&self, board: Board) -> Option<int> {
        ::std::rt::io::timer::sleep(1000); // simulate thinking

        match (*self).strategy {
            LowestAvailable => self.get_lowest_available_index(board.spaces.clone()),
            Minimax         => self.minimax(board)
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

    fn minimax(&self, board: Board) -> Option<int> {
        let scores = self.get_all_scores(board, 0);

        Some(self.index_of_best_score(scores))
    }

    fn get_all_scores(&self, board: Board, depth: int) -> ~[Option<int>] {
        let mut scores: ~[Option<int>] = ~[None, ..9];

        let mut i = 0;
        let available_indexes = board.clone().available_spaces();

        loop {
            if available_indexes.contains(&i) {
                scores[i] = Some(self.score_move(i, board.clone(), depth));
            }

            i += 1;

            if (i as uint) > board.spaces.len() { break }
        }

        scores
    }

    pub fn score_move(&self, index: int, board: Board, depth: int) -> int {
        let new_board = board.place(index);

        self.assign_score_to_board(new_board, depth)
    }

    fn assign_score_to_board(&self, board: Board, depth: int) -> int {
        if board.is_game_over() {
            self.score_finished_board(board.clone(), depth)
        } else {
            self.best_score_from_remaining_spaces(board, depth)
        }
    }

    fn best_score_from_remaining_spaces(&self, board: Board, depth: int) -> int {
        let scores = self.get_all_scores(board.clone(), depth + 1);
        let max_value = scores.iter().max().unwrap().unwrap();

        max_value * -1
    }

    pub fn score_finished_board(&self, board: Board, depth: int) -> int {
        match board.winner() {
            Some(winner) => 9 - depth,
            None         => 0
        }
    }

    fn index_of_best_score(&self, scores: ~[Option<int>]) -> int {
        let unwrapped_scores = self.map_try_unwrap(scores.clone());

        self.index_of_max_score(unwrapped_scores)
    }

    fn index_of_max_score(&self, scores: ~[int]) -> int {
        let max_value = scores.iter().max().unwrap();

        scores.iter().position( |x: &int| *x == *max_value ).unwrap() as int
    }

    fn try_unwrap(&self, box: Option<int>) -> int {
        if box == None {
            -1000
        } else {
            box.unwrap()
        }
    }

    fn map_try_unwrap(&self, scores: ~[Option<int>]) -> ~[int] {
        scores.iter().map(|&score| self.try_unwrap(score)).to_owned_vec()
    }
}

#[cfg(test)]
mod test__minimax {
    use super::*;
    use board::*;

    #[test]
    fn can_score_a_finished_board() {
        let ai = AI::new(Minimax);
        let tie_board = Board::new_from_spaces(~['o','x','o',
                                                 'o','x','x',
                                                 'x','o','x' ]);

        let x_wins_board = Board::new_from_spaces(~['o','x','x',
                                                    'o','o','x',
                                                    'x','o','x' ]);

        let o_wins_board = Board::new_from_spaces(~['o','x','x',
                                                    'x','o','o',
                                                    'x',' ','o' ]);

        let board = Board::new_from_spaces(~['o','x','o',
                                             ' ',' ','x',
                                             ' ',' ',' ' ]);

        let tie_board_score = ai.assign_score_to_board(tie_board, 0);
        let x_wins_score = ai.assign_score_to_board(x_wins_board, 0);
        let o_wins_score = ai.assign_score_to_board(o_wins_board, 0);

        assert!(tie_board_score == 0);
        assert!(x_wins_score > 0);
        assert!(o_wins_score > 0);
    }

    #[test]
    fn can_score_an_almost_finished_board() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             ' ',' ',' ' ]);

        ai.get_all_scores(board.clone(), 0);

        let score = ai.assign_score_to_board(board, 0);

        assert!(score < 0);
    }

//    #[test]
//    fn scores_unavailable_spaces_as_None() {
//        let ai = AI::new(Minimax);
//        let board = Board::new_from_spaces(~['o','x','o',
//                                             'o','x','x',
//                                             ' ',' ',' ' ]);
//
//        assert_eq!(None, ai.score_move(0, board.clone()));
//    }

    #[test]
    fn scores_tying_moves_as_0() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             'x','o',' ' ]);

        assert_eq!(0, ai.score_move(8, board.clone(), 0));
    }

    #[test]
    fn scores_winning_moves_positively() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','o',
                                             'o','x','x',
                                             ' ',' ',' ' ]);

        let winning_score = ai.score_move(7, board.clone(), 0);

        assert!(winning_score > 0);
    }

    #[test]
    fn scores_losing_moves_negatively() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['o','x','x',
                                             'o','x','o',
                                             ' ',' ','x' ]);

        let losing_score = ai.score_move(7, board.clone(), 0);

        assert!(losing_score < 0);
    }

    #[test]
    fn accounts_for_depth() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['x','o','x',
                                             'o','x','o',
                                             ' ',' ',' ' ]);

        let fast_win1 = ai.score_move(6, board.clone(), 0);
        let fast_win2 = ai.score_move(8, board.clone(), 0);
        let slow_win = ai.score_move(7, board.clone(), 0);

        assert!(fast_win1 == fast_win2);
        assert!(fast_win1 > slow_win);
    }

    #[test]
    fn can_pick_the_best_move_from_the_next_turn() {
        let ai = AI::new(Minimax);
        let board = Board::new_from_spaces(~['x','o','x',
                                             'o','x','x',
                                             ' ',' ','o' ]);

        let scores = ai.get_all_scores(board, 0);

        assert_eq!(6, ai.index_of_best_score(scores));
    }
}
