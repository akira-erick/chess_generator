use rand::random_range;
use shakmaty::Move;

use crate::{bots::bot::Bot, game::board::Board};

pub struct RandoBot {
    pub name: String,
}

impl RandoBot {
    pub fn new() -> Self {
        Self {
            name: "RandoBot".to_string(),
        }
    }
}

impl Bot for RandoBot {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_move(&self, board: &Board) -> Option<Move> {
        let legal_moves = board.legal_moves();
        if legal_moves.is_empty() {
            return None;
        }

        let rando_bot = random_range(0..legal_moves.len());
        Some(legal_moves[rando_bot])
    }
}

