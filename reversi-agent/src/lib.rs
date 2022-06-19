mod utils;

use utils::{get_legal_moves, get_move_scores, get_opponent, make_move};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn play(
    player: i8,
    chessboard: &[i8],
    max_depth: u32,
    use_coin_parity: bool,
    use_actual_mobility: bool,
    use_potential_mobility: bool,
    use_corner_score: bool,
    use_stability_score: bool,
) -> usize {
    assert!(chessboard.len() == 64, "number of squares must be 64");

    fn convert_disc(disc: i8) -> Disc {
        if disc > 0 {
            Disc::First
        } else if disc < 0 {
            Disc::Second
        } else {
            Disc::None
        }
    }

    let player = convert_disc(player);
    let mut new_chessboard = [[Disc::None; 8]; 8];
    for (row, chessboard_row) in chessboard.chunks(8).enumerate() {
        for (col, disc) in chessboard_row.into_iter().enumerate() {
            new_chessboard[row][col] = convert_disc(*disc);
        }
    }
    let (move_row, move_col) = AgentPlayer {
        max_depth,
        use_coin_parity,
        use_actual_mobility,
        use_potential_mobility,
        use_corner_score,
        use_stability_score,
    }
    .play(player, &new_chessboard);
    move_row * 8 + move_col
}

#[derive(Clone, Copy, PartialEq)]
pub enum Disc {
    None,
    First,
    Second,
}

struct AgentPlayer {
    max_depth: u32,
    use_coin_parity: bool,
    use_actual_mobility: bool,
    use_potential_mobility: bool,
    use_corner_score: bool,
    use_stability_score: bool,
}

impl AgentPlayer {
    fn play(&self, player: Disc, chessboard: &[[Disc; 8]; 8]) -> (usize, usize) {
        let mut alpha = f64::NEG_INFINITY;
        let mut beta = f64::INFINITY;
        let legal_moves = get_legal_moves(player, chessboard);
        let opponent = get_opponent(player);
        let mut next_chessboard: [[Disc; 8]; 8];
        let mut best_move: Option<(usize, usize)> = None;
        for (player_move, _) in legal_moves.into_iter() {
            next_chessboard = *chessboard;
            make_move(player, player_move, &mut next_chessboard);
            let score =
                self.min_max_search(opponent, &next_chessboard, self.max_depth - 1, alpha, beta);
            match player {
                Disc::First => {
                    if score > alpha {
                        alpha = score;
                        best_move = Some(player_move);
                    }
                }
                Disc::Second => {
                    if score < beta {
                        beta = score;
                        best_move = Some(player_move);
                    }
                }
                _ => panic!("player cannot be none"),
            }
        }
        best_move.unwrap()
    }

    fn min_max_search(
        &self,
        player: Disc,
        chessboard: &[[Disc; 8]; 8],
        depth: u32,
        alpha: f64,
        beta: f64,
    ) -> f64 {
        if depth == 0 {
            return self.get_heuristic(chessboard);
        }
        let legal_moves = get_legal_moves(player, chessboard);
        let opponent = get_opponent(player);
        if legal_moves.len() == 0 {
            let next_legal_moves = get_legal_moves(opponent, chessboard);
            if next_legal_moves.len() == 0 {
                return self.get_heuristic(chessboard);
            }
            return self.min_max_search(opponent, chessboard, depth - 1, alpha, beta);
        }
        let mut next_chessboard: [[Disc; 8]; 8];
        match player {
            Disc::First => {
                let mut alpha = alpha;
                let mut best_score = f64::NEG_INFINITY;
                for (player_move, _) in legal_moves.into_iter() {
                    next_chessboard = *chessboard;
                    make_move(player, player_move, &mut next_chessboard);
                    let score =
                        self.min_max_search(opponent, &next_chessboard, depth - 1, alpha, beta);
                    best_score = best_score.max(score);
                    alpha = alpha.max(score);
                    if score >= beta {
                        break;
                    }
                }
                best_score
            }
            Disc::Second => {
                let mut beta = beta;
                let mut best_score = f64::INFINITY;
                for (player_move, _) in legal_moves.into_iter() {
                    next_chessboard = *chessboard;
                    make_move(player, player_move, &mut next_chessboard);
                    let score =
                        self.min_max_search(opponent, &next_chessboard, depth - 1, alpha, beta);
                    best_score = best_score.min(score);
                    beta = beta.min(score);
                    if score <= alpha {
                        break;
                    }
                }
                best_score
            }
            _ => panic!("player cannot be none"),
        }
    }

    fn get_heuristic(&self, chessboard: &[[Disc; 8]; 8]) -> f64 {
        let mut first_scores = [[0; 8]; 8];
        let mut second_scores = [[0; 8]; 8];
        get_move_scores(Disc::First, chessboard, &mut first_scores);
        get_move_scores(Disc::Second, chessboard, &mut second_scores);

        let mut score = 0.;
        if self.use_coin_parity {
            score += Self::get_coin_parity(chessboard) * 15.;
        }
        if self.use_actual_mobility {
            score += Self::get_actual_mobility(&first_scores, &second_scores) * 2.;
        }
        if self.use_potential_mobility {
            score += Self::get_potential_mobility(chessboard);
        }
        if self.use_corner_score {
            score += Self::get_corner_score(chessboard, &first_scores, &second_scores) * 18.;
        }
        if self.use_stability_score {
            score += Self::get_stability_score(chessboard) * 15.;
        }
        score
    }

    fn get_coin_parity(chessboard: &[[Disc; 8]; 8]) -> f64 {
        let mut coin_parity = 0;
        for chessboard_row in chessboard.iter() {
            for disc in chessboard_row.into_iter() {
                match *disc {
                    Disc::First => coin_parity += 1,
                    Disc::Second => coin_parity -= 1,
                    _ => {}
                }
            }
        }
        coin_parity as f64
    }

    fn get_actual_mobility(first_scores: &[[u32; 8]; 8], second_scores: &[[u32; 8]; 8]) -> f64 {
        let mut first_mobility = 0;
        for score_row in first_scores.iter() {
            for score in score_row.iter() {
                if *score > 0 {
                    first_mobility += 1;
                }
            }
        }
        let mut second_mobility = 0;
        for score_row in second_scores.iter() {
            for score in score_row.iter() {
                if *score > 0 {
                    second_mobility += 1;
                }
            }
        }
        if first_mobility == 0 && second_mobility == 0 {
            0.
        } else {
            let first_mobility = first_mobility as f64;
            let second_mobility = second_mobility as f64;
            (first_mobility - second_mobility) / (first_mobility + second_mobility)
        }
    }

    fn get_potential_mobility(chessboard: &[[Disc; 8]; 8]) -> f64 {
        let mut first_mobility = 0;
        let mut second_mobility = 0;
        let mut first_moves = [[false; 8]; 8];
        let mut second_moves = [[false; 8]; 8];
        let mut update_moves = |row: usize, col: usize, adj_row: isize, adj_col: isize| {
            if !(0 <= adj_row && adj_row < 8) || !(0 <= adj_col && adj_col < 8) {
                return;
            }
            match chessboard[adj_row as usize][adj_col as usize] {
                Disc::First => {
                    let is_potential = &mut second_moves[row][col];
                    if !*is_potential {
                        second_mobility += 1;
                        *is_potential = true;
                    }
                }
                Disc::Second => {
                    let is_potential = &mut first_moves[row][col];
                    if !*is_potential {
                        first_mobility += 1;
                        *is_potential = true;
                    }
                }
                _ => {}
            }
        };
        for row in 0..8 {
            for col in 0..8 {
                if chessboard[row][col] != Disc::None {
                    continue;
                }
                update_moves(row, col, row as isize + 1, col as isize);
                update_moves(row, col, row as isize - 1, col as isize);
                update_moves(row, col, row as isize, col as isize + 1);
                update_moves(row, col, row as isize, col as isize - 1);
                update_moves(row, col, row as isize + 1, col as isize + 1);
                update_moves(row, col, row as isize + 1, col as isize - 1);
                update_moves(row, col, row as isize - 1, col as isize + 1);
                update_moves(row, col, row as isize - 1, col as isize - 1);
            }
        }
        if first_mobility == 0 && second_mobility == 0 {
            0.
        } else {
            let first_mobility = first_mobility as f64;
            let second_mobility = second_mobility as f64;
            (first_mobility - second_mobility) / (first_mobility + second_mobility)
        }
    }

    fn get_corner_score(
        chessboard: &[[Disc; 8]; 8],
        first_scores: &[[u32; 8]; 8],
        second_scores: &[[u32; 8]; 8],
    ) -> f64 {
        let mut first_score = 0;
        let mut second_score = 0;
        for (row, col) in [(0, 0), (0, 7), (7, 0), (7, 7)] {
            match chessboard[row][col] {
                Disc::First => first_score += 2,
                Disc::Second => second_score += 2,
                _ => {
                    if first_scores[row][col] > 0 {
                        first_score += 1;
                    }
                    if second_scores[row][col] > 0 {
                        second_score += 1;
                    }
                }
            }
        }
        if first_score == 0 && second_score == 0 {
            0.
        } else {
            let first_score = first_score as f64;
            let second_score = second_score as f64;
            (first_score - second_score) / (first_score + second_score)
        }
    }

    fn get_stability_score(chessboard: &[[Disc; 8]; 8]) -> f64 {
        const WEIGHTS: [[i32; 8]; 8] = [
            [4, -3, 2, 2, 2, 2, -3, 4],
            [-3, -4, -1, -1, -1, -1, -4, -3],
            [2, -1, 1, 0, 0, 1, -1, 2],
            [2, -1, 0, 1, 1, 0, -1, 2],
            [2, -1, 0, 1, 1, 0, -1, 2],
            [2, -1, 1, 0, 0, 1, -1, 2],
            [-3, -4, -1, -1, -1, -1, -4, -3],
            [4, -3, 2, 2, 2, 2, -3, 4],
        ];
        let mut first_score = 0;
        let mut second_score = 0;
        for (chessboard_row, weight_row) in chessboard.iter().zip(WEIGHTS.iter()) {
            for (disc, weight) in chessboard_row.iter().zip(weight_row.iter()) {
                match *disc {
                    Disc::First => first_score += *weight,
                    Disc::Second => second_score += *weight,
                    _ => {}
                }
            }
        }
        (first_score - second_score) as f64
    }
}
