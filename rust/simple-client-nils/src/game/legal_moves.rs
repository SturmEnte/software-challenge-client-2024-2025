pub mod is_legal;
pub mod legal_advance_moves;

use is_legal::*;
use legal_advance_moves::legal_advance_moves;

use crate::utils::triangular_numbers::calculate_reverse_triangular_number_floor;

use super::{board::Board, game_state::GameState, moves::{CarrotsToExchange, GameMove}};

pub fn calculate_legal_moves(game_state: &GameState, board: &Board) -> Vec<GameMove>{
    let (current_hare, opponent_hare) = if game_state.get_next_team() == game_state.team {
        (&game_state.your_hare, &game_state.opponent_hare)} 
    else {
        (&game_state.opponent_hare, &game_state.your_hare)
    };

    let mut legal_moves = Vec::new();

    if is_eat_salad_legal(current_hare, board) {
        legal_moves.push(GameMove::EatSalad);
        return legal_moves;
    }

    if is_fall_back_legal(current_hare.position, opponent_hare.position) {
        legal_moves.push(GameMove::FallBack);
    }

    if is_exchange_carrots_legal(current_hare.position, board) {
        legal_moves.push(GameMove::ExchangeCarrots(CarrotsToExchange::PlusTen));
        if current_hare.carrots >= 10 {legal_moves.push(GameMove::ExchangeCarrots(CarrotsToExchange::PlusTen));}
    }

    //println!("carrots: {}", current_hare.carrots);
    for distance in 1..=calculate_reverse_triangular_number_floor(current_hare.carrots) {
        if current_hare.position + distance as u8 > 64 {break;}
        legal_advance_moves(&mut legal_moves, current_hare, opponent_hare, board, distance as u8, 0, false, game_state.last_carrot_swap, game_state.turn);
    }
    return legal_moves;
}
