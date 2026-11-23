use shakmaty::{Chess, Move, Position, Square};

use crate::{bots::bot::Bot, game::board::Board};

const CHECKMATE_SCORE: i32 = 10000;

const DRAW_SCORE: i32 = 0;

pub struct MinimaxBot {
    pub name: String,
    pub depth: u8,
}

impl MinimaxBot {
    pub fn new(depth: u8) -> Self {
        Self {
            name: "MinimaxBot".to_string(),
            depth,
        }
    }

    fn piece_value(role: shakmaty::Role) -> i32 {
        match role {
            shakmaty::Role::Pawn => 1,
            shakmaty::Role::Knight => 3,
            shakmaty::Role::Bishop => 3,
            shakmaty::Role::Rook => 5,
            shakmaty::Role::Queen => 9,
            shakmaty::Role::King => 100,
        }
    }

    fn evaluate_position(board: &shakmaty::Board, side: shakmaty::Color) -> i32 {
        let mut score = 0;

        for square in Square::ALL {
            if let Some(piece) = board.piece_at(square) {
                let value = Self::piece_value(piece.role);

                if piece.color == side {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
        score
    }

    fn minimax(chess: &Chess, depth: u8, side: shakmaty::Color) -> i32 {
        let legal_moves = chess.legal_moves();
        let maximizing = chess.turn() == side;

        if chess.is_checkmate() {
            return if maximizing {
                -CHECKMATE_SCORE - depth as i32
            } else {
                CHECKMATE_SCORE + depth as i32
            };
        }

        if chess.is_insufficient_material() {
            return DRAW_SCORE;
        }

        if depth == 0 {
            return Self::evaluate_position(&chess.board(), side);
        }

        if maximizing {
            let mut max_eval = i32::MIN;

            for mv in legal_moves {
                let mut new_board = chess.clone();
                new_board.play_unchecked(mv);
                let eval = Self::minimax(&new_board, depth - 1, side);
                max_eval = max_eval.max(eval);
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for mv in legal_moves {
                let mut new_board = chess.clone();
                new_board.play_unchecked(mv);
                let eval = Self::minimax(&new_board, depth - 1, side);
                min_eval = min_eval.min(eval);
            }
            min_eval
        }
    }
}

impl Bot for MinimaxBot {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_move(&self, board: &Board) -> Option<Move> {
        let chess = board.position();
        let legal_moves = chess.legal_moves();

        if legal_moves.is_empty() {
            return None;
        }

        let side = chess.turn();

        let mut best_score = i32::MIN;
        let mut best_moves = Vec::new();

        for mv in legal_moves {
            let mut new_chess = chess.clone();
            new_chess.play_unchecked(mv);

            let score = Self::minimax(&new_chess, self.depth.saturating_sub(1), side);
            if score > best_score {
                best_score = score;
                best_moves.clear();
                best_moves.push(mv);
            } else if score == best_score {
                best_moves.push(mv);
            }
        }

        let index = rand::random_range(0..best_moves.len());

        Some(best_moves[index])
    }
}