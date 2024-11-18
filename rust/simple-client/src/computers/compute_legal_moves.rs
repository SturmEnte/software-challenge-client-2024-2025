use crate::enums::field_type::FieldType;
// use crate::structs::action::Advance;
use crate::GameData;
use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;

// A array with the distances from 1 to 44 with their carrot costs
const RENNKARTE : [u16; 44] = [1,3,6,10,15,21,28,36,45,55,66,78,91,105,120,136,153,171,190,210,231,253,276,300,325,351,378,406,435,465,496,528,561,595,630,666,703,741,780,820,861,903,946,990];

pub fn compute_legal_moves(game_data: &GameData) -> Vec<Box<dyn Move>> {
    let mut legal_moves: Vec<Box<dyn Move>> = Vec::new();
    
    // Calculate moves that just advance
    // Iterate through all distances from 1 to 44
    for distance in 1..45 {
        // Check if our hare has enough carrots to move the distance
        if (game_data.our_hare.carrots as u16) < RENNKARTE[distance as usize - 1] {
            continue; // Move is invalid
        }

        // Check if enemy hare is on the new field after the move
        if game_data.our_hare.position == game_data.enemy_hare.position + distance {
            continue; // Move is invalid
        }

        let new_field: FieldType = game_data.board.get_field((game_data.our_hare.position + distance) as usize).unwrap();

        print!("Distance: {}", distance);
        match new_field {
            FieldType::Position1 | FieldType::Position2 | FieldType::Carrots => {
                // legal_moves.push(Move::new(vec![Box::new(Advance::new(distance))]));
                legal_moves.push(Box::new(AdvanceMove::new(distance)));
                println!(" - legal");
            },
            _ => {
                println!(" - illegal");
                continue; // Move is invalid
                // Some moves are not computed yet
            }
        }
    }

    legal_moves
}