use crate::enums::card::Card;
use crate::enums::card::card_to_string;
use crate::enums::move_type::MoveType;

pub trait Move {
    fn to_string(&self) -> String; 
    fn get_type(&self) -> MoveType;
}

// Advance Move
pub struct AdvanceMove {
    pub distance: u8,
    pub card: Option<Card>,
}

impl AdvanceMove {
    pub fn new(distance: u8, card: Option<Card>) -> AdvanceMove {
        AdvanceMove {
            distance: distance,
            card: card,
        }
    }
}

impl Move for AdvanceMove {
    fn to_string(&self) -> String {

        if self.card.is_some() {
            return format!("<data class=\"advance\" distance=\"{}\"><card>{}</card></data>", self.distance, card_to_string(self.card.as_ref().unwrap()));
        }

        format!("<data class=\"advance\" distance=\"{}\"/>", self.distance)
    }

    fn get_type(&self) -> MoveType {
        MoveType::Advance
    }
}

// Fallback Move
pub struct FallbackMove {}

impl FallbackMove {
    pub fn new() -> FallbackMove {
        FallbackMove {}
    }
}

impl Move for FallbackMove {
    fn to_string(&self) -> String {
        format!("<data class=\"fallback\"/>")
    }

    fn get_type(&self) -> MoveType {
        MoveType::Fallback
    }
}

// Eat Salad Move
pub struct EatSaladMove {}

impl EatSaladMove {
    pub fn new() -> EatSaladMove {
        EatSaladMove {}
    }
}

impl Move for EatSaladMove {
    fn to_string(&self) -> String {
        format!("<data class=\"eatsalad\"/>")
    }

    fn get_type(&self) -> MoveType {
        MoveType::EatSalad
    }
}

// Exchange Carrots Move
pub struct ExchangeCarrotsMove {
    pub amount: i8,
}

impl ExchangeCarrotsMove {
    pub fn new(amount: i8) -> ExchangeCarrotsMove {
        ExchangeCarrotsMove {amount: amount}
    }
}

impl Move for ExchangeCarrotsMove {
    fn to_string(&self) -> String {
        format!("<data class=\"exchangecarrots\" amount=\"{}\"/>", self.amount)
    }

    fn get_type(&self) -> MoveType {
        MoveType::ExchangeCarrots
    }
}