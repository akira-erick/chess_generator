use shakmaty::{Color, Move, Position};

use crate::game::{board::Board, history::{History, HistoryEntry}};

pub struct Game {
    board: Board,
    history: History,
    status: GameStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            history: History::new(),
            status: GameStatus::Ongoing,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        if self.status != GameStatus::Ongoing {
            return Err("Game is already over".to_string());
        }

        let player = self.board.side_to_move();

        let move_number = (self.history.len() / 2) as u32 + 1;

        self.board.make_move(mv)?;

        let fen = self.board.to_fen();

        self.history.add(HistoryEntry {
            move_number,
            player,
            played_move: mv,
            fen,
        });

        self.update_status();

        Ok(())
    }

    fn update_status(&mut self) {
        let position = self.board.position();

        if position.is_checkmate() {
            self.status = match self.board.side_to_move() {
                Color::White => GameStatus::BlackWon,
                Color::Black => GameStatus::WhiteWon,
            };
        } else if position.is_stalemate() || position.is_insufficient_material() {
            self.status = GameStatus::Draw;
        } else {
            self.status = GameStatus::Ongoing;
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}