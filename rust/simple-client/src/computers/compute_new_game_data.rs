use crate::structs::{game_data::GameData, game_move::Move};

pub fn compute_new_game_data(game_data: &GameData, m: &Box<dyn Move>) -> GameData {
    let mut new_game_data = game_data.clone();

    

    new_game_data
}