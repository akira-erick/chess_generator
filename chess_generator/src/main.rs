use shakmaty::Color;

use crate::{bots::{greedy_bot::GreedyBot, rando_bot::RandoBot}, engine::controller::{Controller, MatchError}, game::game::GameStatus, rendering::rendering::print_match};

mod game;
mod bots;
mod engine;
mod rendering;

fn main() {
    let white = Box::new(RandoBot::new());
    let black = Box::new(GreedyBot::new());

    let mut controller = Controller::new(white, black);

    let result = controller.run();

    match result {
        Ok(status) => {

            println!("Game finished with status: {:?}", status);
            if status == GameStatus::WhiteWon || status == GameStatus::BlackWon {
                let winner = if status == GameStatus::WhiteWon { controller.get_bot(Color::White) } else { controller.get_bot(Color::Black) };
                println!("The winner is: {}", winner.name());
            }
            
            let history = controller.game().history();
            println!("Game history:");
            print_match(history);
        }
        Err(e) => {
            match e {
                MatchError::BotFailed { bot_name } => {
                    println!("A bot failed to make a move: {}", bot_name);
                }
                MatchError::IllegalMove { bot_name, chess_error } => {
                    println!("A bot made an illegal move: {}. Error: {}", bot_name, chess_error);
                }
            }
        },
    }
}
