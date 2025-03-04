use crate::game::{board::Board, game_state::GameState, moves::GameMove};

pub trait ComputerPlayer {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove;
}