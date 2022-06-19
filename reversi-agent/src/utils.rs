use crate::Disc;

pub fn get_opponent(player: Disc) -> Disc {
    match player {
        Disc::First => Disc::Second,
        Disc::Second => Disc::First,
        _ => panic!("player cannot be none"),
    }
}

pub fn get_legal_moves(player: Disc, chessboard: &[[Disc; 8]; 8]) -> Vec<((usize, usize), u32)> {
    let mut scores = [[0; 8]; 8];
    get_move_scores(player, chessboard, &mut scores);
    let mut legal_moves = Vec::new();
    for (row, score_row) in scores.into_iter().enumerate() {
        for (col, score) in score_row.into_iter().enumerate() {
            if score > 0 {
                legal_moves.push(((row, col), score));
            }
        }
    }
    legal_moves.sort_by(|(_, a_score), (_, b_score)| b_score.cmp(a_score));
    legal_moves
}

pub fn get_move_scores(player: Disc, chessboard: &[[Disc; 8]; 8], scores: &mut [[u32; 8]; 8]) {
    for i in 0..8 {
        update_scores_bidirectional(player, chessboard, scores, (0..8).map(|j| (i, j)));
        update_scores_bidirectional(player, chessboard, scores, (0..8).map(|j| (j, i)));
    }
    update_scores_bidirectional(player, chessboard, scores, (0..8).map(|i| (i, i)));
    update_scores_bidirectional(player, chessboard, scores, (0..8).map(|i| (i, 7 - i)));
    for i in 1..6 {
        update_scores_bidirectional(player, chessboard, scores, (i..8).map(|j| (j, j - i)));
        update_scores_bidirectional(player, chessboard, scores, (i..8).map(|j| (j - i, j)));
        update_scores_bidirectional(player, chessboard, scores, (i..8).map(|j| (j, 7 - (j - i))));
        update_scores_bidirectional(player, chessboard, scores, (i..8).map(|j| (j - i, 7 - j)));
    }
}

fn update_scores_bidirectional<T: DoubleEndedIterator<Item = (usize, usize)> + Clone>(
    player: Disc,
    chessboard: &[[Disc; 8]; 8],
    scores: &mut [[u32; 8]; 8],
    line: T,
) {
    update_scores_unidirectional(player, chessboard, scores, line.clone());
    update_scores_unidirectional(player, chessboard, scores, line.rev());
}

#[derive(Clone, Copy)]
enum ScoreUpdaterState {
    MatchNone,
    MatchSelf,
    MatchOpponent,
}

fn update_scores_unidirectional<T: Iterator<Item = (usize, usize)>>(
    player: Disc,
    chessboard: &[[Disc; 8]; 8],
    scores: &mut [[u32; 8]; 8],
    line: T,
) {
    use ScoreUpdaterState::*;
    let mut state = MatchNone;
    let mut score = 0;
    for (row, col) in line {
        let disc = chessboard[row][col];
        match state {
            MatchNone => {
                if disc == player {
                    state = MatchSelf;
                }
            }
            MatchSelf => {
                if disc == Disc::None {
                    state = MatchNone;
                } else if disc != player {
                    state = MatchOpponent;
                    score = 1;
                }
            }
            MatchOpponent => {
                if disc == Disc::None {
                    scores[row][col] += score;
                    state = MatchNone;
                    score = 0;
                } else if disc == player {
                    state = MatchSelf;
                    score = 0;
                } else {
                    score += 1;
                }
            }
        }
    }
}

pub fn make_move(
    player: Disc,
    (move_row, move_col): (usize, usize),
    chessboard: &mut [[Disc; 8]; 8],
) {
    let fwd_row = (move_row + 1)..8;
    let bwd_row = (0..move_row).rev();
    let fwd_col = (move_col + 1)..8;
    let bwd_col = (0..move_col).rev();
    chessboard[move_row][move_col] = player;
    flip_discs_in_direction(
        player,
        chessboard,
        fwd_row.clone().map(|row| (row, move_col)),
    );
    flip_discs_in_direction(
        player,
        chessboard,
        bwd_row.clone().map(|row| (row, move_col)),
    );
    flip_discs_in_direction(
        player,
        chessboard,
        fwd_col.clone().map(|col| (move_row, col)),
    );
    flip_discs_in_direction(
        player,
        chessboard,
        bwd_col.clone().map(|col| (move_row, col)),
    );
    flip_discs_in_direction(player, chessboard, fwd_row.clone().zip(fwd_col.clone()));
    flip_discs_in_direction(player, chessboard, fwd_row.clone().zip(bwd_col.clone()));
    flip_discs_in_direction(player, chessboard, bwd_row.clone().zip(fwd_col.clone()));
    flip_discs_in_direction(player, chessboard, bwd_row.clone().zip(bwd_col.clone()));
}

fn flip_discs_in_direction<T: Iterator<Item = (usize, usize)> + Clone>(
    player: Disc,
    chessboard: &mut [[Disc; 8]; 8],
    line: T,
) {
    if !has_flips_in_direction(player, chessboard, line.clone()) {
        return;
    }
    for (row, col) in line {
        let disc = &mut chessboard[row][col];
        if *disc == player {
            return;
        } else {
            *disc = player;
        }
    }
}

fn has_flips_in_direction<T: Iterator<Item = (usize, usize)>>(
    player: Disc,
    chessboard: &[[Disc; 8]; 8],
    line: T,
) -> bool {
    for (row, col) in line {
        let disc = chessboard[row][col];
        if disc == Disc::None {
            return false;
        }
        if disc == player {
            return true;
        }
    }
    false
}
