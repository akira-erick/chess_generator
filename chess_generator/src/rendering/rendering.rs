use shakmaty::{CastlingMode, Chess, Position, Square, fen::Fen};

use crate::game::history::History;

pub fn print_match(history: &History) {
     for entry in history.entries() {
         println!(
             "{}. {}: {} (FEN: {})",
             entry.move_number, entry.player, entry.san, entry.fen
         );

         print_board(&entry.fen);
         print!("-------------------------\n");
     }
}

fn print_board(fen: &str)  {
    let fen: Fen = fen.parse().unwrap();

    let position: Chess = fen.into_position(CastlingMode::Standard).unwrap();

    let board = position.board();

    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = Square::new((rank * 8 + file) as u32);

            match board.piece_at(square) {
                Some(piece) => print!("{} ", piece.char()),
                None => print!(". "),
            }
        }

        println!("");
    }

}