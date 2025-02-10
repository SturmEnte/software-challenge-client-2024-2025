use crate::game::{field_type::FieldType, game_state::GameState, moves::GameMove};

pub trait ComputerPlayer {
    fn make_move(&mut self, bord: &[FieldType; 65], game_state: &GameState) -> GameMove;
}