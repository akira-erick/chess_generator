use shakmaty::{CastlingMode, Chess, Color, Position, Square, fen::Fen};

use crate::game::history::History;

pub fn to_pgn(history: &History) -> String {
    let mut pgn = String::new();

    pgn.push_str("[Event \"Automated Chess Match\"]\n");
    pgn.push_str("[Site \"Local\"]\n");
    pgn.push_str("[Date \"????.??.??\"]\n");
    pgn.push_str("[Round \"1\"]\n");
    pgn.push_str("[White \"Engine 1\"]\n");
    pgn.push_str("[Black \"Engine 2\"]\n");
    pgn.push_str("[Result \"*\"]\n\n");

    for entry in history.entries() {
        if entry.player == Color::White {
            pgn.push_str(&format!(
                "{}. {} ",
                entry.move_number,
                entry.san
            ));
        } else {
            pgn.push_str(&format!("{} ", entry.san));
        }
    }

    pgn.push_str("*");

    pgn
}

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