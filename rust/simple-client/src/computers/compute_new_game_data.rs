use crate::{enums::move_type::MoveType, structs::{game_data::GameData, game_move::{AdvanceMove, Move}, hare::Hare}};

pub fn compute_new_game_data(game_data: &GameData, m: &Box<dyn Move>, our_hares_move: &bool) -> GameData {
    let mut new_game_data: GameData = game_data.clone();

    let current_hare: &mut Hare = if *our_hares_move { &mut new_game_data.our_hare } else { &mut new_game_data.enemy_hare };
    let other_hare: &mut Hare = if *our_hares_move { &mut new_game_data.enemy_hare } else { &mut new_game_data.our_hare };

    match m.get_type() {
        MoveType::Advance => {
            let advance_move = m.as_any().downcast_ref::<AdvanceMove>().unwrap();
            current_hare.position += advance_move.distance;
        },
        MoveType::EatSalad => {

        },
        MoveType::ExchangeCarrots => {

        },
        MoveType::Fallback => {

        }
    }

    new_game_data
}