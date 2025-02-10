use crate::utils::triangular_numbers::calculate_triangular_number;

use super::{cards::Card, field_type::FieldType, game_error::GameError, game_state::GameState, moves::GameMove};

impl GameState {
    /// A Function that updates the GameState based on a `GameMove`.
    pub fn update(&mut self, bord: &[FieldType; 65], mov: &GameMove) -> Result<(), GameError> {
        self.turn += 1;
        let current_team = self.get_current_team();
        let opponent_position = self.get_hare_by_team(&!current_team).position;
        let current_hare = self.get_hare_by_team_mut(&current_team);

        match bord[current_hare.position as usize] {
            FieldType::Position1 => if opponent_position < current_hare.position {current_hare.carrots += 10},
            FieldType::Position2 => if opponent_position > current_hare.position {current_hare.carrots += 30},
            _ => {},
        }

        current_hare.ate_salad_last_round = false;
        
        match mov {
            GameMove::FallBack => {
                let mut hedgehog_position = current_hare.position;
                while hedgehog_position > 0 {
                    hedgehog_position -= 1;
                    if bord[hedgehog_position as usize] == FieldType::Hedgehog {
                        current_hare.carrots += (current_hare.position - hedgehog_position) as u16 * 10;
                        current_hare.position = hedgehog_position;
                        break;
                    }   
                };
            },
            GameMove::EatSalad => {
                current_hare.eat_salad(opponent_position)?;
                current_hare.ate_salad_last_round = true;
            },
            GameMove::ExchangeCarrots(carrots_to_exchange) => {
                match carrots_to_exchange {
                    super::moves::CarrotsToExchange::MinusTen => current_hare.carrots -= 10,
                    super::moves::CarrotsToExchange::PlusTen => current_hare.carrots += 10,
                }
            },
            GameMove::Advance(distance) => {
                if current_hare.position + distance > 64 {return Err(GameError::OutOfBounce);}
                current_hare.position += distance;
                current_hare.carrots -= calculate_triangular_number(*distance as u16);
            },
            GameMove::AdvanceWithCards(distance, jumps, last_card) => {
                if current_hare.position + distance > 64 {return Err(GameError::OutOfBounce);}
                current_hare.position += distance;
                current_hare.carrots -= calculate_triangular_number(*distance as u16);

                if jumps.get_number_of_jumps() > 0 {
                    //Specifies whether the hare should hurry ahead or fall back.
                    // true = hurry ahead
                    // false = fall back
                    let mut hurry_ahead_or_fall_back = jumps.is_first_card_hurry_ahead();
                    if jumps.get_number_of_jumps() % 2 != 0 {hurry_ahead_or_fall_back = !hurry_ahead_or_fall_back}
                    if hurry_ahead_or_fall_back {
                        current_hare.position = opponent_position + 1;
                    } else {
                        current_hare.position = opponent_position - 1;
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
                                current_hare.use_card(&Card::SwapCarrots)?;
                                let your_hare_carrots = self.your_hare.carrots;
                                self.your_hare.carrots = self.opponent_hare.carrots;
                                self.opponent_hare.carrots = your_hare_carrots;
                            },
                            Card::EatSalad => {
                                if current_hare.salads < 1 {return Err(GameError::NoSalads);}
                                current_hare.use_card(&Card::EatSalad)?;
                                current_hare.eat_salad(opponent_position)?;
                            },
                            Card::FallBack => {
                                current_hare.use_card(&Card::FallBack)?;
                                current_hare.position = opponent_position - 1;
                            },
                            Card::HurryAhead => {
                                current_hare.use_card(&Card::HurryAhead)?;
                                if opponent_position == 63 {return Err(GameError::OutOfBounce)};
                                current_hare.position = opponent_position + 1;
                            },
                        }  
                    } 
                    _ => return Err(GameError::CanNotUseCardsHere)
                }
            },
        }
        println!("{:?}", self);
        return Ok(());
    }
}