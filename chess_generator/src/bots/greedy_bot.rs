use rand::random_range;
use shakmaty::{Move, Position, Role};

use crate::{bots::bot::Bot, game::board::Board};

pub struct GreedyBot {
    pub name: String,
}

impl GreedyBot {
    pub fn new() -> Self {
        Self {
            name: "GreedyBot".to_string(),
        }
    }

    fn piece_value(role: Role) -> i32 {
        match role {
            Role::Pawn => 1,
            Role::Knight => 3,
            Role::Bishop => 3,
            Role::Rook => 5,
            Role::Queen => 9,
            Role::King => 100,
        }
    }

    fn captured_piece_value(position: &shakmaty::Board, mv: Move) -> i32 {
        match mv {
            Move::Normal {capture, promotion, .. } => {
                if let Some(role) = promotion {
                    GreedyBot::piece_value(role)
                } else if let Some(role) = capture {
                    GreedyBot::piece_value(role)
                } else {
                    0
                }
            }
            Move::EnPassant { to, from: _ } => {
                if let Some(piece) = position.piece_at(to) {
                    GreedyBot::piece_value(piece.role)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

impl Bot for GreedyBot {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_move(&self, board: &Board) -> Option<Move> {
        let board = board.clone();

        let legal_moves = board.legal_moves();
        let position = board.position();

        if legal_moves.is_empty() {
            return None;
        }

        let mut best_value = 0;
        let mut best_moves = Vec::new();

        for mv in legal_moves {
            let value = GreedyBot::captured_piece_value(&position.board(), mv);

            if value > best_value {
                best_value = value;
                best_moves.clear();
                best_moves.push(mv);
            } else if value == best_value {
                best_moves.push(mv);
            }
        }
        
        let rng = random_range(0..best_moves.len());
        let selected_move = best_moves[rng];

        return Some(selected_move);
    }
}