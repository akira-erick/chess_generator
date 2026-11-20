use crate::game::{board::Board, history::History};

pub struct Game {
    board: Board,
    history: History,
    status: GameStatus,
}

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
}