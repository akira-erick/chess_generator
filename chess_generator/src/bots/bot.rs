use shakmaty::Move;

use crate::game::board::Board;

pub trait Bot {
    fn name(&self) -> &str;
    fn make_move(&self, board: &Board) -> Option<Move>;
}