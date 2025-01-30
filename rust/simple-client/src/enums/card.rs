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

pub fn string_to_card(string: &str) -> Card {

    println!("Try to convert this string to a card: {}", string);

    match string {
        "SWAP_CARROTS" => return Card::SwapCarrots,
        "HURRY_AHEAD" => return Card::HurryAhead,
        "FALL_BACK" => return Card::FallBack,
        "EAT_SALAD" => return Card::EatSalad,
        _ => panic!("Invalid card string: {}", string)
    }
}