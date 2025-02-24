use std::fmt::Display;

use super::cards::Card;

/// Enum representing different types of moves
#[derive(Debug)]
pub enum GameMove {
    FallBack,
    EatSalad,
    ExchangeCarrots(CarrotsToExchange),
    Advance(u8),
    AdvanceWithCards(u8, JumpCardDetails, Card)
}
/// Enum representing the types of carrot exchanges that are possible.
#[derive(Debug)]
pub enum CarrotsToExchange {
    MinusTen,
    PlusTen,
}

/// Struct that holds details about the number and sequence of jump cards played in a move 
/// (By jump cards we mean the "Hurry Ahead" and "Fall Back" cards as these allow the player to jump forwards or backwards).
#[derive(Debug)]
pub struct JumpCardDetails {
    first_card_and_number_of_cards: u8 // A single byte that stores both if the first card is a "Hurry Ahead" card or a "Fall Back" card in it's MSB and the number of jump cards in the rest of the bits.
}

impl Display for GameMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMove::FallBack => write!(f, "⏪ Fall back"),
            GameMove::EatSalad => write!(f, "🍴 Eat salad"),
            GameMove::ExchangeCarrots(carrots_to_exchange) => write!(f, "🔄 Carrots to exchange: {}", carrots_to_exchange),
            GameMove::Advance(i) => write!(f, "⏩ Advance by: {}", i),
            GameMove::AdvanceWithCards(i, jump_card_details, card) => write!(f, "⏭ Advanced by {} with {}\n with last card: {}", i, jump_card_details, card),
        }
    }
}

impl Display for CarrotsToExchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CarrotsToExchange::MinusTen => "-10",
            CarrotsToExchange::PlusTen => "+10",
        })?;
        Ok(())
    }
}

impl Display for JumpCardDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jump Cards: {}", self.get_number_of_jumps())?;
        write!(f, "First card is: {}", if self.is_first_card_hurry_ahead() {"hurry ahead"} else {"fall back"})?;
        Ok(())
    }
}


impl JumpCardDetails {
    /// Creates a new JumpCardDetails instance.
    /// 
    /// # Parameters
    /// - `first_card_hurry_ahead`: A boolean indicating if the first card is a "Hurry Ahead" card.
    /// - `jumps`: The number of jumps, which must be less than 128.
    ///
    /// # Panics
    /// This function will panic if `jumps` is 128 or greater.
    pub fn new(first_card_hurry_ahead: bool, jumps: u8) -> Self {
        //Only numbers from 0-127 are allowed because Jumps has to fit into seven bits.
        assert!(jumps < 128);
        let first_card_and_number_of_cards = (first_card_hurry_ahead as u8) << 7 | jumps;
        return JumpCardDetails {first_card_and_number_of_cards}
    }

    /// Creates a JumpCardDetails instance from a list of cards.
    ///
    /// # Parameters
    /// - `card_list`: A reference to a vector of Cards (The cards must be playable in this order).
    ///
    /// # Returns
    /// A JumpCardDetails instance based on the list of cards..
    pub fn from_card_list(card_list: &Vec<Card>) -> Self {
        if card_list.len() < 1 {
            return JumpCardDetails::new(false, 0)
        } else {
            if card_list[0] == Card::HurryAhead {
                return JumpCardDetails::new(true, (card_list.len() - 1) as u8);
            }
            return JumpCardDetails::new(false, (card_list.len() - 1) as u8);
        }
    }

    /// Checks if the first card is a "Hurry Ahead" card.
    ///
    /// # Returns
    /// A boolean indicating whether the first card is a "Hurry Ahead" card.
    /// true: The first card is a "Hurry Ahead" card.
    /// false: The first card is a "Falle Back" card.
    pub fn is_first_card_hurry_ahead(&self) -> bool {
        self.first_card_and_number_of_cards & 0b10000000 != 0
    }

    /// Gets the number of jumps stored in the JumpCardDetails.
    ///
    /// # Returns
    /// The number of jumps as a u8.
    pub fn get_number_of_jumps(&self) -> u8 {
        self.first_card_and_number_of_cards & 0b01111111
    }
}