// This file contains the logic of the client
// It calculates which move should be made next

use crate::structs::game_data::GameData;
use crate::structs::game_move::Move;

use crate::computers::compute_legal_moves::compute_legal_moves;

pub fn compute_move(game_data: &GameData) -> Box<dyn Move> {
    let mut moves = compute_legal_moves(&game_data);

    // Check if there are any legal moves
    if moves.len() == 0 {
        panic!("No legal moves found");
    }

    // Select a random valid move
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..moves.len() as u32);

    let random_move: Box<dyn Move> = moves.remove(random_number as usize);
    return random_move;
}