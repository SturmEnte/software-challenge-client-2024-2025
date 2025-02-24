use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Card {
    SwapCarrots,
    EatSalad,
    FallBack,
    HurryAhead
}

impl Card {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "SWAP_CARROTS" => Some(Card::SwapCarrots),
            "EAT_SALAD" => Some(Card::EatSalad),
            "FALL_BACK" => Some(Card::FallBack),
            "HURRY_AHEAD" => Some(Card::HurryAhead),
            _ => None
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::SwapCarrots => write!(f, "🔃Card: Swap Carrots"),
            Card::EatSalad => write!(f, "🍽️Card: Eat Salad"),
            Card::FallBack => write!(f, " 🏃🏻‍♀️Card: Fall Back"),
            Card::HurryAhead => write!(f, "🏃🏻‍♀️‍➡️Card: Hurry Ahead"),
        }
    }
}

impl Card {
    pub fn convert_to_string(&self) -> String {
        match self {
            Card::SwapCarrots => String::from("SWAP_CARROTS"),
            Card::EatSalad => String::from("EAT_SALAD"),
            Card::FallBack => String::from("FALL_BACK"),
            Card::HurryAhead => String::from("HURRY_AHEAD"),
        }
    }
}