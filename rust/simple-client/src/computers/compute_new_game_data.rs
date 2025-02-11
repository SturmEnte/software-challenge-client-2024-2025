use crate::structs::{game_data::GameData, game_move::Move};

pub fn compute_new_game_data(game_data: &GameData, m: &Box<dyn Move>, our_hares_move: &bool) -> GameData {
    let mut new_game_data = game_data.clone();

    println!("Our hare's move: {}", our_hares_move);

    new_game_data
}