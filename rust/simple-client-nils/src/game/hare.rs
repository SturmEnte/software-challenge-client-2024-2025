use std::fmt::Display;

use crate::utils::triangular_numbers::calculate_triangular_number;

use super::{cards::Card, game_error::GameError};

#[derive(Debug)]
pub struct Hare {
    pub ate_salad_last_round: bool,
    pub position: u8,
    pub salads: u8,
    pub carrots: u16,
    pub card_swap_carrots: u8,
    pub card_eat_salad: u8,
    pub card_fall_back: u8,
    pub card_hurry_ahead: u8,
}

impl Display for Hare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    📍 Position: {}\n", self.position)?;
        write!(f, "    🥕 Carrots: {}\n", self.carrots)?;
        write!(f, "    🥬 Salads: {}\n", self.salads)?;
        write!(f, "    😋 Ate Salad Last Round: {}\n", self.ate_salad_last_round)?;
        write!(f, "    🔃 Card Swap Carrots: {}\n", self.card_swap_carrots)?;
        write!(f, "    🍽️ Eat Salad Cards: {}\n", self.card_eat_salad)?;
        write!(f, "    ⏪ Fall Back Cards: {}\n", self.card_fall_back)?;
        write!(f, "    ⏩ Hurry Ahead Cards: {}", self.card_hurry_ahead)?;
        Ok(())
    }
}

impl Hare {
    pub fn new() -> Self {
        Hare {
            position: 0,
            salads: 5,
            carrots: 68,
            ate_salad_last_round: false,
            card_swap_carrots: 0,
            card_eat_salad: 0,
            card_fall_back: 0,
            card_hurry_ahead: 0,
        }
    }

    pub fn add_card(&mut self, card: &Card) {
        match card {
            Card::SwapCarrots => self.card_swap_carrots += 1,
            Card::EatSalad => self.card_eat_salad += 1,
            Card::FallBack => self.card_fall_back += 1,
            Card::HurryAhead => self.card_hurry_ahead +=1,
        }
    }

    /// Consumes a specified card, decrementing its count in the game state.
    ///
    /// This function makes the Hare to consume one of its cards (depending on the card parameter).
    /// This function dose not check if the Hare hase cards.
    /// This function ony reduces the number of one of the card types that the Hare has by one.
    ///
    /// # Parameters
    ///
    /// - `card`: A reference to the `Card` enum that represents the card to be consumed.
    pub fn consume_card(&mut self, card: &Card) {
        match card {
            Card::SwapCarrots => {
                self.card_swap_carrots -= 1;
            },
            Card::EatSalad => {
                self.card_eat_salad -= 1
            },
            Card::FallBack => {
                self.card_fall_back -= 1
            },
            Card::HurryAhead => {
                self.card_hurry_ahead -=1
            },
        }
    }

    /// Makes the `Hare` eat a salad and  gives it carrots based on its and its opponent's position.
    ///
    /// This function checks if the hare has at least one salad.
    /// If the hare has no salads, it returns an error indicating that there are no salads available.
    /// If the hare has a salad, it decrements the number of salads by one and increases the hare's
    /// carrot count based on its position relative to the opponent's position:
    /// - If the hare's position is greater than the opponent's position, it gains 10 carrots.
    /// - If the hare's position is less than or equal to the opponent's position, it gains 30 carrots.
    ///
    /// Note: This function does not check if the hare is on a field where it can actually eat a salad.
    ///
    /// # Arguments
    ///
    /// * `opponent_position` - A `u8` representing the position of the opponent.
    ///
    /// # Returns
    ///
    /// * `Result<(), GameError>` - Returns `Ok(())` if the hare successfully eats a salad,
    ///   or an error of type `GameError` if the hare has no salads to eat.
    ///
    /// # Errors
    ///
    /// * `GameError::NoSalads` - Returned when the hare has no carrots available to eat a salad.
    pub fn eat_salad(&mut self, opponent_position: u8) -> Result<(), GameError> {
        if self.salads == 0 {return Err(GameError::NoSalads);}
        self.salads -= 1;
        if self.position > opponent_position {
            self.carrots += 10;
        } else {
            self.carrots += 30;
        }
        return Ok(())
    }

    /// Advances the position of the Hare by a distance.
    /// 
    /// Increases the position of the Hare by the distance and deducts carrots accordingly.
    /// This function does not check whether the player can stand on the new field!
    ///
    /// # Parameters
    /// - `distance`: A `u8` representing the distance to advance. This value must be
    ///   within the bounds of the game board.
    ///
    /// # Returns
    /// - `Result<(), GameError>`: Returns `Ok(())` if the advancement is successful.
    ///   - `GameError::OutOfBounce`: Returned if advancing the position would exceed
    ///     the maximum position of 64 on the game board.
    ///   - `GameError::NotEnoughCarrots`: Returned if the Hare does not have enough
    ///     carrots to pay for the advancement.
    pub fn advance(&mut self, distance: u8) -> Result<(), GameError> {
        if self.position + distance > 64 {return Err(GameError::OutOfBounce);}
        if calculate_triangular_number(distance as u16) > self.carrots {return Err(GameError::NotEnoughCarrots);}
        self.position += distance;
        self.carrots -= calculate_triangular_number(distance as u16);
        Ok(())
    }
}

