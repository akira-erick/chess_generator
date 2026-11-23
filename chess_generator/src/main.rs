use crate::{bots::rando_bot::RandoBot, engine::controller::Controller, game::game::GameStatus, rendering::rendering::print_match};

mod game;
mod bots;
mod engine;
mod rendering;

fn main() {
    let white = Box::new(RandoBot::new());
    let black = Box::new(RandoBot::new());

    let mut controller = Controller::new(white, black);

    let result = controller.run();

    match result {
        Ok(status) => {
            println!("Game finished with status: {:?}", status);
            if status == GameStatus::Draw {
                println!("The game ended in a draw.");
            } else {
                let winner = if status == GameStatus::WhiteWon { "White" } else { "Black" };
                println!("The winner is: {}", winner);
            }

            let history = controller.game().history();
            println!("Game history:");
            print_match(history);
            }
        Err(_) => todo!(),
    }
}
