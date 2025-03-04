use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
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
            Card::SwapCarrots => write!(f, "SWAP_CARROTS"),
            Card::EatSalad => write!(f, "EAT_SALAD"),
            Card::FallBack => write!(f, "FALL_BACK"),
            Card::HurryAhead => write!(f, "HURRY_AHEAD"),
        }
    }
}