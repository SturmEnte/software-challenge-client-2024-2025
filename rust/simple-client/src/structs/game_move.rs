use crate::enums::card::Card;
use crate::enums::card::card_to_string;
use crate::enums::move_type::MoveType;

pub trait Move {
    fn to_string(&self) -> String; 
    fn get_type(&self) -> MoveType;
    fn as_any(&self) -> &dyn std::any::Any;
}

// Advance Move
pub struct AdvanceMove {
    pub distance: u8,
    pub cards: Option<Vec<Card>>,
}

impl AdvanceMove {
    pub fn new(distance: u8, cards: Option<Vec<Card>>) -> AdvanceMove {
        AdvanceMove {
            distance: distance,
            cards: cards,
        }
    }
}

impl Move for AdvanceMove {
    fn to_string(&self) -> String {

        if self.cards.is_some() {
            let mut cards_string = String::new();

            for card in self.cards.as_ref().unwrap() {
                cards_string.push_str(&card_to_string(card));
            }

            return format!("<data class=\"advance\" distance=\"{}\">{}</data>", self.distance, cards_string);
        }

        format!("<data class=\"advance\" distance=\"{}\"/>", self.distance)
    }

    fn get_type(&self) -> MoveType {
        MoveType::Advance
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}