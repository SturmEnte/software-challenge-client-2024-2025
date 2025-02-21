use crate::game::{board::Board, field_type::FieldType, hare::Hare};

pub(super) fn give_position_field_reward(board: &Board, current_hare: &mut Hare, opponent_position: u8) {
    match board.board[current_hare.position as usize] {
        FieldType::Position1 if opponent_position < current_hare.position => current_hare.carrots += 10,
        FieldType::Position2 if opponent_position > current_hare.position => current_hare.carrots += 30,
        _ => {},
    }
}