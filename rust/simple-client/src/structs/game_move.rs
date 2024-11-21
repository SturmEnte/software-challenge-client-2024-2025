pub trait Move {
    fn to_string(&self) -> String; 
}

// Advance Move
pub struct AdvanceMove {
    pub distance: u8,
}

impl AdvanceMove {
    pub fn new(distance: u8) -> AdvanceMove {
        AdvanceMove {
            distance: distance,
        }
    }
}

impl Move for AdvanceMove {
    fn to_string(&self) -> String {
        format!("<data class=\"advance\" distance=\"{}\"/>", self.distance)
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
}

// Old code befor the docs were updated
// use crate::structs::action::Action;

// pub struct Move {
//     pub actions: Vec<Box<dyn Action>>,
// }

// impl Move {
//     pub fn new(actions: Vec<Box<dyn Action>>) -> Move {
//         Move {
//             actions: actions,
//         }
//     }
// }

// Old action file:
/*pub trait Action {
    fn to_string(&self, index: &i8) -> String; 
}

pub struct Advance {
    pub distance: i8,
}

impl Advance {
    pub fn new(distance: i8) -> Advance {
        Advance {
            distance: distance,
        }
    }
}

impl Action for Advance {
    fn to_string(&self, index: &i8) -> String {
        format!("<advance order=\"{}\" distance=\"{}\"/>", index, self.distance)
    }
}*/