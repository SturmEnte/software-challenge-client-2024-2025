use crate::game::{Board, GameMove, Hare, FieldType, JumpCardDetails, Card};
use crate::game::legal_moves::is_carrot_swap_legal;
use crate::utils::triangular_numbers::calculate_triangular_number;

/// Calculates and addes the legal advance moves to the legal moves Vector.
///
/// # Parameters
/// - `legal_moves`: A mutable reference to a vector that the legal moves will be added to.
/// - `current_hare`: A reference to the `Hare` struct representing the current player's hare.
/// - `opponent_hare`: A reference to the `Hare` struct representing the opponent's hare.
/// - `board`: A reference to the `Board` struct representing the game board.
/// - `distance`: The distance the current hare intends to move.
/// - `jumps`: The number of jumps that have been made so far in the current turn by the current hare.
/// - `first_jump_card_hurry_ahead`: A boolean indicating if the first jump card is a "Hurry Ahead" card.
/// - `last_carrot_swap`: An unsigned integer representing the turn in which the last carrot swap has taken place.
/// - `turn`: An unsigned integer representing the current turn.
pub fn legal_advance_moves(legal_moves: &mut Vec<GameMove>, current_hare: &Hare, opponent_hare: &Hare, board: &Board, distance: u8, jumps: u8, first_jump_card_hurry_ahead: bool, last_carrot_swap: u8, turn: u8) {
    if jumps == 0 {
        let new_position = current_hare.position + distance;

        if new_position == opponent_hare.position && opponent_hare.position != 64 {return;}
        match board.board[(new_position as u8) as usize] {
            FieldType::Position1 => legal_moves.push(GameMove::Advance(distance as u8)),
            FieldType::Position2 => legal_moves.push(GameMove::Advance(distance as u8)),
            FieldType::Carrots => legal_moves.push(GameMove::Advance(distance as u8)),
            FieldType::Salad => if current_hare.salads > 0 {legal_moves.push(GameMove::Advance(distance as u8));},
            FieldType::Market => if current_hare.carrots >= 10 + calculate_triangular_number(distance as u16) {
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::EatSalad));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::SwapCarrots));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::FallBack));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::HurryAhead));
            }
            FieldType::Hare => {
                if current_hare.card_eat_salad != 0 && current_hare.salads != 0 {legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::EatSalad));}
                if current_hare.card_swap_carrots != 0 && is_carrot_swap_legal(current_hare, opponent_hare, last_carrot_swap, turn) {legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::SwapCarrots));}
                if current_hare.card_hurry_ahead != 0 && opponent_hare.position < 64 && opponent_hare.position > new_position {legal_advance_moves(legal_moves, current_hare, opponent_hare, board, distance, 1, true, last_carrot_swap, turn);}
                if current_hare.card_fall_back != 0 && opponent_hare.position > 0 && opponent_hare.position < new_position {legal_advance_moves(legal_moves, current_hare, opponent_hare, board, distance, 1, false, last_carrot_swap, turn);}
            }
            FieldType::Goal => {if current_hare.salads == 0 && current_hare.carrots <= 10 + calculate_triangular_number(distance as u16) {legal_moves.push(GameMove::Advance(distance as u8));}},
            _ => {}
        }
    } else {
        let last_card_hurry_ahead = is_last_card_hurry_ahead(jumps, first_jump_card_hurry_ahead);
        let new_position = if last_card_hurry_ahead {
            opponent_hare.position + 1
        } else {
            opponent_hare.position - 1
        };

        match board.board[(new_position as u8) as usize] {
            FieldType::Position1 => legal_moves.push(GameMove::AdvanceWithCards(distance, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps - 1), if last_card_hurry_ahead {Card::HurryAhead} else {Card::FallBack})),
            FieldType::Position2 => legal_moves.push(GameMove::AdvanceWithCards(distance, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps - 1), if last_card_hurry_ahead {Card::HurryAhead} else {Card::FallBack})),
            FieldType::Carrots => legal_moves.push(GameMove::AdvanceWithCards(distance, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps - 1), if last_card_hurry_ahead {Card::HurryAhead} else {Card::FallBack})),
            FieldType::Salad => if current_hare.salads > 0 {legal_moves.push(GameMove::AdvanceWithCards(distance, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps - 1), if last_card_hurry_ahead {Card::HurryAhead} else {Card::FallBack}));},
            FieldType::Market => if current_hare.carrots >= 10 + calculate_triangular_number(distance as u16) {
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::EatSalad));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::SwapCarrots));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::FallBack));
                legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::HurryAhead));
            }
            FieldType::Hare => {
                if current_hare.card_eat_salad != 0 && current_hare.salads != 0 {legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::EatSalad));}
                if current_hare.card_swap_carrots != 0 && is_carrot_swap_legal(current_hare, opponent_hare, last_carrot_swap, turn) {legal_moves.push(GameMove::AdvanceWithCards(distance as u8, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps), Card::SwapCarrots));}
                if current_hare.card_hurry_ahead - number_of_hurry_ahead_cards(jumps, first_jump_card_hurry_ahead) != 0 && opponent_hare.position < 64 && opponent_hare.position > new_position {legal_advance_moves(legal_moves, current_hare, opponent_hare, board, distance, jumps + 1, first_jump_card_hurry_ahead, last_carrot_swap, turn);}
                if current_hare.card_fall_back - number_of_fall_back_cards(jumps, first_jump_card_hurry_ahead) != 0 && opponent_hare.position > 0 && opponent_hare.position < new_position {legal_advance_moves(legal_moves, current_hare, opponent_hare, board, distance, jumps + 1, first_jump_card_hurry_ahead, last_carrot_swap, turn);}
            }
            FieldType::Goal => {if current_hare.salads == 0 && current_hare.carrots  <= 10 + calculate_triangular_number(distance as u16) {legal_moves.push(GameMove::AdvanceWithCards(distance, JumpCardDetails::new(first_jump_card_hurry_ahead, jumps - 1), if last_card_hurry_ahead {Card::HurryAhead} else {Card::FallBack}));}},
            _ => {}
        }
    }
}

fn is_last_card_hurry_ahead(jumps: u8, first_jump_card_hurry_ahead: bool) -> bool {
    (jumps % 2 == 0) ^ first_jump_card_hurry_ahead
}

fn number_of_hurry_ahead_cards(jumps: u8, first_jump_card_hurry_ahead: bool) -> u8 {
    if first_jump_card_hurry_ahead {
        (jumps as f64 / 2.0).ceil() as u8
    } else {
        (jumps as f64 / 2.0).floor() as u8
    }
}

fn number_of_fall_back_cards(jumps: u8, first_jump_card_hurry_ahead: bool) -> u8 {
    if first_jump_card_hurry_ahead {
        (jumps as f64 / 2.0).floor() as u8
    } else {
        (jumps as f64 / 2.0).ceil() as u8
    }
}