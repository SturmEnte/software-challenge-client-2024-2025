use crate::utils::triangular_numbers::calculate_triangular_number;

use super::{field_type::FieldType, game_error::GameError, game_state::GameState, moves::GameMove};

impl GameState {
        //TODO: Reuse parts for the is legal functions and then remove the rest.
        pub fn update_checked(&mut self, bord: &[FieldType; 65], mov: GameMove) -> Result<(), GameError> {
            let current_team = self.get_current_team();
            let opponent_position = self.get_hare_by_team(&!current_team).position;
            let current_hare = self.get_hare_by_team_mut(&current_team);
    
            match bord[current_hare.position as usize] {
                FieldType::Position1 => if current_hare.position > opponent_position {current_hare.carrots += 10},
                FieldType::Position2 => if current_hare.position < opponent_position {current_hare.carrots += 30},
                _ => {},
            }
    
            match mov {
                GameMove::FallBack => {
                    current_hare.ate_salad_last_round = false;
                    let mut hedgehog_position = current_hare.position;
                    while hedgehog_position > 0 {
                        hedgehog_position -= 1;
                        if bord[hedgehog_position as usize] == FieldType::Hedgehog {
                            if opponent_position == hedgehog_position {return Err(GameError::FieldIsOccupied)}
                            current_hare.carrots += (current_hare.position - hedgehog_position * 10) as u16;
                            current_hare.position = hedgehog_position;
                            break;
                        }   
                    };
                },
                GameMove::EatSalad => {
                    if current_hare.ate_salad_last_round {return Err(GameError::CanNotEatSaladsTwiceInARow);}
                    if bord[current_hare.position as usize] != FieldType::Salad {return Err(GameError::NoEatingSaladsHere);}
                    current_hare.salads -= 1;
                    if current_hare.position > opponent_position {
                        current_hare.carrots += 10;
                    } else {
                        current_hare.carrots += 30;
                    }
                    current_hare.ate_salad_last_round = true;
                },
                GameMove::ExchangeCarrots(carrots) => {
                    current_hare.ate_salad_last_round = false;
                    if bord[current_hare.position as usize] != FieldType::Carrots {return Err(GameError::CanNotExchangeCarrotsHere);}
                    match carrots {
                        super::moves::CarrotsToExchange::MinusTen => current_hare.carrots -= 10,
                        super::moves::CarrotsToExchange::PlusTen => current_hare.carrots += 10,
                    }
                },
                GameMove::Advance(distance) => {
                    current_hare.ate_salad_last_round = false;
                    let carrot_cost = calculate_triangular_number(distance as u16);
                    let new_position = current_hare.position + distance;
    
                    if current_hare.carrots < carrot_cost {return Err(GameError::NotEnoughCarrots);}
                    current_hare.can_stand_on_without_cards(&bord[new_position as usize], carrot_cost)?;
                    if new_position == opponent_position {return Err(GameError::FieldIsOccupied);}
                    current_hare.position = new_position;
                    current_hare.carrots -= carrot_cost;
    
                },
                GameMove::AdvanceWithCards(_, _, _) => todo!(),
            };
            self.turn += 1;
            return Ok(())
        }
}