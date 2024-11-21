use std::any::Any;

use crate::enums::field_type::FieldType;
use crate::enums::move_type::MoveType;

use crate::GameData;

use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;
use crate::structs::game_move::FallbackMove;
use crate::structs::game_move::EatSaladMove;
use crate::structs::game_move::ExchangeCarrotsMove;

// A array with the distances from 1 to 44 with their carrot costs
const RENNKARTE : [u16; 44] = [1,3,6,10,15,21,28,36,45,55,66,78,91,105,120,136,153,171,190,210,231,253,276,300,325,351,378,406,435,465,496,528,561,595,630,666,703,741,780,820,861,903,946,990];

pub fn compute_legal_moves(game_data: &GameData) -> Vec<Box<dyn Move>> {
    let mut legal_moves: Vec<Box<dyn Move>> = Vec::new();

    // Check if eating a salad is possible
    // A salad move is possible if our hare is on a salad field and it has moved on it in the last move
    if game_data.our_hare.salads > 0 
        && game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Salad 
        && game_data.our_hare.last_move.is_some() 
        && game_data.our_hare.last_move_type == Some(MoveType::Advance) {      // This line does not work    
        
        legal_moves.push(Box::new(EatSaladMove::new()));
        return legal_moves;
    }

    // Check if carrot exchange is possible
    if game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Carrots {
        legal_moves.push(Box::new(ExchangeCarrotsMove::new(10)));

        if game_data.our_hare.carrots >= 10 {
            legal_moves.push(Box::new(ExchangeCarrotsMove::new(-10)));
        }
    }

    // Loop through all fields that are behind the hare, starting at the first field behind the hare
    // until the closest hedgehog field is found and then checking if our hare can fallback on it
    for i in (1..game_data.our_hare.position).rev() {
        println!("{}",i);
        
        // If the current field is nota hedgehog field, continue with the next field
        if game_data.board.board[i as usize].unwrap() != FieldType::Hedgehog {                               
            continue;
        }

        // Check validity of the fallback move and add the fallbackmove to the legal moves
        if game_data.our_hare.position > i      // Is the enemy hare not on this hedgehog field
        && game_data.enemy_hare.position != i { // Is the hedgehog behind our hare
            legal_moves.push(Box::new(FallbackMove::new()));
        }

        // If a hedgehog field is found, stop the loop not matter if a falback is possible or not
        break;
    }

    // Calculate moves that just advance
    // Iterate through all distances from 1 to 44
    for distance in 1..45 {

        // Check if the hare would still be on the game board after moving that distance
        if game_data.our_hare.position + distance > 44 {
            continue; // Move is invalid
        }

        // Check if our hare has enough carrots to move the distance
        if (game_data.our_hare.carrots as u16) < RENNKARTE[distance as usize - 1] {
            continue; // Move is invalid
        }

        // Check if the enemy hare is on the new field after the move
        if game_data.our_hare.position + distance == game_data.enemy_hare.position {
            continue; // Move is invalid
        }

        let new_field: FieldType = game_data.board.get_field((game_data.our_hare.position + distance) as usize).unwrap();

        match new_field {
            FieldType::Position1 | FieldType::Position2 | FieldType::Carrots => {
                // legal_moves.push(Move::new(vec![Box::new(Advance::new(distance))]));
                legal_moves.push(Box::new(AdvanceMove::new(distance)));
            },
            FieldType::Salad => {
                if game_data.our_hare.salads > 0 {
                    legal_moves.push(Box::new(AdvanceMove::new(distance)));
                }
            },
            _ => {
                continue; // Move is invalid
                // Some moves are not computed yet
            }
        }
    }

    legal_moves
}