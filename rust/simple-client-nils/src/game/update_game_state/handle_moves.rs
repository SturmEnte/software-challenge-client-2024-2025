use crate::game::{cards::Card, field_type::FieldType, game_error::GameError, hare::Hare, moves::{CarrotsToExchange, JumpCardDetails}};

use super::handle_cards::*;

pub(super) fn handle_move_eat_salad(current_hare: &mut Hare, opponent_position: u8) -> Result<(), GameError>{
    current_hare.eat_salad(opponent_position)?;
    current_hare.ate_salad_last_round = true;
    Ok(())
}

pub(super) fn handle_move_exchange_carrots(current_hare: &mut Hare, carrots_to_exchange: &CarrotsToExchange){
    match carrots_to_exchange {
        CarrotsToExchange::MinusTen => current_hare.carrots -= 10,
        CarrotsToExchange::PlusTen => current_hare.carrots += 10,
    }
}

pub fn handle_move_fall_back(current_hare: &mut Hare, bord: &[FieldType; 65]) {
    let mut hedgehog_position = current_hare.position;
    while hedgehog_position > 0 {
        hedgehog_position -= 1;
        if bord[hedgehog_position as usize] == FieldType::Hedgehog {
            current_hare.carrots += (current_hare.position - hedgehog_position) as u16 * 10;
            current_hare.position = hedgehog_position;
            break;
        }   
    };
}

pub(super) fn handle_move_advance(current_hare: &mut Hare, distance: u8) -> Result<(), GameError> {
    current_hare.advance(distance)?;
    Ok(())
}

pub(super) fn handle_move_advance_with_cards(current_hare: &mut Hare, opponent_hare: &mut Hare, bord: &[FieldType; 65], distance: u8, jumps: &JumpCardDetails, last_card: &Card, current_turn: &u8, last_carrot_swap: &mut u8) -> Result<(), GameError> {
    current_hare.advance(distance)?;

    if jumps.get_number_of_jumps() > 0 {
        //Specifies whether the hare should hurry ahead or fall back.
        // true = hurry ahead
        // false = fall back
        let mut hurry_ahead_or_fall_back = jumps.is_first_card_hurry_ahead();
        if jumps.get_number_of_jumps() % 2 == 0 {hurry_ahead_or_fall_back = !hurry_ahead_or_fall_back}
        if hurry_ahead_or_fall_back {
            current_hare.position = opponent_hare.position + 1;
        } else {
            current_hare.position = opponent_hare.position - 1;
        }
    }

    match bord[current_hare.position as usize] {
        FieldType::Market => {
            current_hare.carrots -= 10;
            current_hare.add_card(last_card);
        },
        FieldType::Hare =>  {
            match last_card {
                Card::SwapCarrots => {
                    handle_card_swap_carrots(current_hare, opponent_hare, last_carrot_swap, current_turn)?;
                },
                Card::EatSalad => {
                    handle_card_eat_salad(current_hare, opponent_hare.position)?;
                },
                Card::FallBack => {
                    handle_card_fall_back(current_hare, opponent_hare.position)?;
                },
                Card::HurryAhead => {
                    handle_card_hurry_ahead(current_hare, opponent_hare.position)?;
                },
            }  
        } 
        _ => return Err(GameError::CanNotUseCardsHere)
    }
    Ok(())
}
