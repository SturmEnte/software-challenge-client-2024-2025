
use super::{cards::Card, field_type::FieldType, game_error::GameError};

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
    /// If the Hare attempts to consume a card that is not available, an error will be returned.
    /// This function ony reduces the number of one of the card types that the Hare has by one.
    ///
    /// # Parameters
    ///
    /// - `card`: A reference to the `Card` enum that represents the card to be consumed.
    ///
    /// # Returns
    ///
    /// - `Result<(), GameError>`: Returns `Ok(())` if the card was successfully consumed. If the card
    ///   cannot be consumed due to insufficient quantity, it returns a `GameError` indicating the
    ///   specific issue (e.g., `MissingCardSwapCarrots`, `MissingCardEatSalad`, etc.).
    ///
    /// # Errors
    ///
    /// This function may return the following errors:
    /// - `GameError::MissingCardSwapCarrots`: Returned if there are no remaining Swap Carrots cards.
    /// - `GameError::MissingCardEatSalad`: Returned if there are no remaining Eat Salad cards.
    /// - `GameError::MissingCardFallBack`: Returned if there are no remaining Fall Back cards.
    /// - `GameError::MissingCardHurryAhead`: Returned if there are no remaining Hurry Ahead cards.
    pub fn consume_card(&mut self, card: &Card) -> Result<(), GameError> {
        match card {
            Card::SwapCarrots => {
                if self.card_swap_carrots == 0 {return Err(GameError::MissingCardSwapCarrots)}
                self.card_swap_carrots -= 1;
            },
            Card::EatSalad => {
                if self.card_swap_carrots == 0 {return Err(GameError::MissingCardEatSalad)}
                self.card_eat_salad -= 1
            },
            Card::FallBack => {
                if self.card_swap_carrots == 0 {return Err(GameError::MissingCardFallBack)}
                self.card_fall_back -= 1
            },
            Card::HurryAhead => {
                if self.card_swap_carrots == 0 {return Err(GameError::MissingCardHurryAhead)}
                self.card_hurry_ahead -=1
            },
        }
        Ok(())
    }

    pub fn can_stand_on_without_cards(&self, field: &FieldType, carrot_cost: u16) -> Result<(), GameError> {
        match field {
            FieldType::Start => return Err(GameError::CanNotReturnToStart),
            FieldType::Hedgehog => return Err(GameError::EnterdHedgehogFieldWhileMovingForward),
            FieldType::Salad => if self.salads < 1 {return Err(GameError::NoSalads);},
            FieldType::Market => return Err(GameError::NoCardPurchased),
            FieldType::Hare => return Err(GameError::NoCardPlayd),
            FieldType::Goal => {
                if self.salads > 0 {return Err(GameError::TooManySalads);}
                if self.carrots > 10 - carrot_cost {return Err(GameError::TooManyCarrots);}
            },
            _ => {}
        }
        return Ok(());
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
}