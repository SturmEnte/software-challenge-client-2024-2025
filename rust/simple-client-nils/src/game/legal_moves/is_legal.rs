use crate::game::{Hare, FieldType, Board, FIRST_HEADGEHOG, HEDGEHOG_FIELDS};

/// Checks if it is legal for the hare to eat salad.
///
/// # Parameters
///
/// - `current_hare`: A reference to a `Hare` struct.
/// - `board`: A reference to a `Board` struct.
///
/// # Returns
///
/// Returns `true` if the hare can legally eat salad (i.e., the current position is
/// a salad field and the hare has not eaten salad in the last round). Returns `false`
/// otherwise.
pub fn is_eat_salad_legal(current_hare: &Hare, board: &Board) -> bool {
    return board.board[current_hare.position as usize] == FieldType::Salad && !current_hare.ate_salad_last_round;
}

/// Checks if exchanging carrots is legal for the hare.
///
/// # Parameters
///
/// - `current_hare_position`: A `u8` representing the current position of the hare
///   on the board.
/// - `board`: A reference to a `Board` struct that contains the game board.
///
/// # Returns
///
/// Returns `true` if the field at the hare's current position is of type `FieldType::Carrots`.
/// Returns `false` otherwise.
pub fn is_exchange_carrots_legal(current_hare_position: u8, board: &Board) -> bool {
    return board.board[current_hare_position as usize] == FieldType::Carrots;
}

/// Determines if it is legal for the hare to fall back.
///
/// # Parameters
///
/// - `current_hare_position`: A `u8` representing the current position of the hare.
/// - `opponent_position`: A `u8` representing the current position of the opponent.
///
/// # Returns
///
/// Returns `true` if it is legal for the hare to fall back,
/// and `false` otherwise.
pub fn is_fall_back_legal(current_hare_position: u8, opponent_position: u8) -> bool {
    if current_hare_position > FIRST_HEADGEHOG {
        match HEDGEHOG_FIELDS.binary_search(&current_hare_position) {
            Err(i) => {
                return opponent_position != HEDGEHOG_FIELDS[(i - 1) as usize];
            }
            Ok(i) if i > 0 => return opponent_position != HEDGEHOG_FIELDS[(i - 1) as usize],
            _ => {}
        }
    }
    return false
}

/// Determines if a carrot swap is legal.
/// 
/// This function dose not check if the Hare has a swap carrot cart.
///
/// # Parameters
///
/// - `hare_1`: A reference to the first `Hare` instance. This represents the first hare involved in the swap.
/// - `hare_2`: A reference to the second `Hare` instance. This represents the second hare involved in the swap.
/// - `last_carrot_swap`: A `u8` representing the turn number of the last carrot swap that occurred.
/// - `turn`: A `u8` representing the current turn number.
///
/// # Returns
///
/// Returns `true` if the carrot swap is legal, and `false` otherwise.
pub fn is_carrot_swap_legal(hare_1: &Hare, hare_2: &Hare, last_carrot_swap: u8, turn: u8) -> bool {
    if hare_1.position >= 57 || hare_2.position >= 57 {return false;}
    if last_carrot_swap + 2 >= turn || last_carrot_swap == 0 {return false;}
    return true 
}