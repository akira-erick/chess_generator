use shakmaty::Color;

use crate::{bots::bot::{self, Bot}, game::game::{Game, GameStatus}};

pub struct Controller {
    game: Game,
    white: Box<dyn Bot>,
    black: Box<dyn Bot>,
}

#[derive(Debug)]
pub enum MatchError {
    BotFailed {
        bot_name: String,
    },
    IllegalMove {
        bot_name: String,
        chess_error: String,
    },
}

impl Controller {
    pub fn new(white: Box<dyn Bot>, black: Box<dyn Bot>) -> Self {
        Controller { game: Game::new(), white, black }
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn get_bot(&self, color: Color) -> &dyn Bot {
        match color {
            Color::White => self.white.as_ref(),
            Color::Black => self.black.as_ref(),
        }
    }

    pub fn play_turn(&mut self) -> Result<(), MatchError> {
        let current_color = self.game.board().side_to_move();
        let bot = self.get_bot(current_color);
        let bot_name = bot.name().to_string();  

        let mv = bot.make_move(self.game.board())
            .ok_or_else(|| MatchError::BotFailed { bot_name: bot_name.clone() })?;

        self.game.make_move(mv.clone())
            .map_err(|e| MatchError::IllegalMove { bot_name: bot_name.clone(), chess_error: e.to_string() })?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<GameStatus, MatchError> {
        while *self.game.status() == GameStatus::Ongoing {
            self.play_turn()?;
        }

        Ok(self.game.status().clone())
    }
}