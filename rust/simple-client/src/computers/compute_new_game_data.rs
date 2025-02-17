use colored::Colorize;

use crate::{enums::move_type::MoveType, structs::{game_data::GameData, game_move::{AdvanceMove, Move}, hare::Hare}, utils::triangular_number::triangular_number};

pub fn compute_new_game_data(game_data: &GameData, m: &Box<dyn Move>, our_hares_move: &bool) -> GameData {
    let mut new_game_data: GameData = game_data.clone();

    // let current_hare: &mut Hare = if *our_hares_move { &mut new_game_data.our_hare } else { &mut new_game_data.enemy_hare };
    // let other_hare: &mut Hare = if *our_hares_move { &mut new_game_data.enemy_hare } else { &mut new_game_data.our_hare };

    let mut current_hare: Hare = if *our_hares_move { new_game_data.our_hare.clone() } else { new_game_data.enemy_hare.clone() };
    let mut other_hare: Hare = if *our_hares_move { new_game_data.enemy_hare.clone() } else { new_game_data.our_hare.clone() };


    match m.get_type() {
        MoveType::Advance => {
            let advance_move = m.as_any().downcast_ref::<AdvanceMove>().unwrap();
            
            current_hare.position += advance_move.distance;
            current_hare.carrots -= triangular_number(advance_move.distance.into());

            if advance_move.card.is_some() {
                println!("{}", "Don't know how to simulate a card".red());
            }
        },
        MoveType::EatSalad => {
            current_hare.salads -= 1;
        },
        MoveType::ExchangeCarrots => {
            println!("{}", "Don't know how to simulate exchange carrots".red());
        },
        MoveType::Fallback => {
            println!("{}", "Don't know how to simulate fallback".red());
        }
    }

    if *our_hares_move {
        new_game_data.our_hare = current_hare;
        new_game_data.enemy_hare = other_hare;
    } else {
        new_game_data.our_hare = other_hare;
        new_game_data.enemy_hare = current_hare;
    }

    new_game_data
}