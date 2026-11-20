use shakmaty::{Chess, Move, Position};

pub struct Board {
    position: Chess
}

impl Board {
    pub fn new() -> Self {
        Self {
            position: Chess::default()
        }
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        if !self.position.is_legal(mv) {
            return Err("Illegal move".to_string());
        }

        self.position.play_unchecked(mv);

        Ok(())
    }

    pub fn position(&self) -> &Chess {
        &self.position
    }

    pub fn is_game_over(&self) -> bool {
        self.position.is_game_over()
    }
}