#[derive(PartialEq)]
pub enum Card {
    SwapCarrots,
    HurryAhead,
    FallBack,
    EatSalad
}

pub fn card_to_string(card: &Card) -> String {
    match card {
        Card::SwapCarrots => return String::from("SWAP_CARROTS"),
        Card::HurryAhead => return String::from("HURRY_AHEAD"),
        Card::FallBack => return String::from("FALL_BACK"),
        Card::EatSalad => return String::from("EAT_SALAD"),
    }
}