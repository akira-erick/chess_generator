use std::{str::FromStr};

use shakmaty::{Chess, Color, Move, Position, fen::Fen, CastlingMode};

pub struct Board {
    position: Chess
}

impl Board {
    pub fn new() -> Self {
        Self {
            position: Chess::default()
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let fen = Fen::from_str(fen)
            .map_err(|e| format!("Invalid FEN: {}", e))?;

        let position: Chess = fen
            .into_position(CastlingMode::Standard)
            .map_err(|e| format!("Invalid chess position: {}", e))?;

        Ok(Self { position })
    }

    pub fn position(&self) -> &Chess {
        &self.position
    }

    pub fn side_to_move(&self) -> Color {
        self.position.turn()
    }

    pub fn to_fen(&self) -> String {
        Fen::from_position(&self.position.clone(), shakmaty::EnPassantMode::Legal)
            .to_string()
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.position.legal_moves().into_iter().collect()
    }

    pub fn is_legal_move(&self, mv: Move) -> bool {
        self.position.is_legal(mv)
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        if !self.position.is_legal(mv) {
            return Err("Illegal move".to_string());
        }

        self.position.play_unchecked(mv);

        Ok(())
    }

    pub fn is_check(&self) -> bool {
        self.position.is_check()
    }

    pub fn is_checkmate(&self) -> bool {
        self.position.is_checkmate()
    }

    pub fn is_stalemate(&self) -> bool {
        self.position.is_stalemate()
    }

    pub fn is_insufficient_material(&self) -> bool {
        self.position.is_insufficient_material()
    }

    pub fn is_game_over(&self) -> bool {
        self.position.is_game_over()
    }

    pub fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
        }
    }
}