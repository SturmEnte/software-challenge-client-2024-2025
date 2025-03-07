use crate::game::{cards::Card, game_error::GameError, hare::Hare};

pub(super) fn handle_card_hurry_ahead(current_hare: &mut Hare, opponent_position: u8) -> Result<(), GameError> {
    current_hare.consume_card(&Card::HurryAhead);
    if opponent_position == 64 {return Err(GameError::OutOfBounce)};
    current_hare.position = opponent_position + 1;
    Ok(())
}

pub(super) fn handle_card_fall_back(current_hare: &mut Hare, opponent_position: u8) -> Result<(), GameError> {
    current_hare.consume_card(&Card::FallBack);
    current_hare.position = opponent_position - 1;
    Ok(())
}

pub(super) fn handle_card_eat_salad(current_hare: &mut Hare, opponent_position: u8) -> Result<(), GameError> {
    current_hare.consume_card(&Card::EatSalad);
    current_hare.eat_salad(opponent_position)?;
    Ok(())
}

pub(super) fn handle_card_swap_carrots(current_hare: &mut Hare, opponent_hare: &mut Hare, last_carrot_swap: &mut u8, current_turn: &u8) -> Result<(), GameError> {
    current_hare.consume_card(&Card::SwapCarrots);
    *last_carrot_swap = *current_turn;
    let current_hare_carrots = current_hare.carrots;
    current_hare.carrots = opponent_hare.carrots;
    opponent_hare.carrots = current_hare_carrots;
    Ok(())
}