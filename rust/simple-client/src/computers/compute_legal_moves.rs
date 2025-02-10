use crate::enums::card::card_to_string;
use crate::enums::field_type::FieldType;
use crate::enums::move_type::MoveType;

use crate::utils::triangular_number::triangular_number;
use crate::GameData;

use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;
use crate::structs::game_move::FallbackMove;
use crate::structs::game_move::EatSaladMove;
use crate::structs::game_move::ExchangeCarrotsMove;

use crate::enums::card::Card;

pub fn compute_legal_moves(game_data: &GameData) -> Vec<Box<dyn Move>> {
    let mut legal_moves: Vec<Box<dyn Move>> = Vec::new();

    if game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Salad {
        println!("We are on a salad field");
        if game_data.our_hare.last_move.is_some() {
            println!("We have a last move");
            if game_data.our_hare.last_move_type == Some(MoveType::Advance) {
                println!("Our last move was an advance move");
            }
        }
    }  

    // Eat salad move
    // Check if the last move was an advance move and if the hare is on a salad field
    // If so, is the hare forced to eat a salad
    if  game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Salad    // Check if the current field is a salad field
        && game_data.our_hare.last_move.is_some()                                                   // Check if the hare has a last move
        && game_data.our_hare.last_move_type == Some(MoveType::Advance) {                           // Check if the hare's last move was an advance move    

        legal_moves.push(Box::new(EatSaladMove::new()));
        // This line returns the legal moves array with only the eat salad move and so the hare is forced to eat a salad

        return legal_moves;
    }

    // Exchange carrots move
    // Check if the hare is on a carrot field
    if game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Carrots {
        // If so add a exchange carrots move to the legal moves that adds 10 carrots to the hare's carrots
        legal_moves.push(Box::new(ExchangeCarrotsMove::new(10)));

        // Also add a exchange carrots move that subtracts 10 carrots from the hare's carrots if the hare has at least 10 carrots
        if game_data.our_hare.carrots >= 10 {
            legal_moves.push(Box::new(ExchangeCarrotsMove::new(-10)));
        }
    }

    // Fallback move
    // Loop through all fields that are behind the hare, starting at the first field behind the hare
    // until the closest hedgehog field is found and then checking if our hare can fallback on it
    for i in (1..game_data.our_hare.position).rev() {
        if crate::DEBUGGING {
            println!("{}",i);
        }
        
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

    // Advance move
    // Iterate through all distances from 1 to 44 and evaluate if the move is possible
    for distance in 1..(crate::FIELD_COUNT as u8 - game_data.our_hare.position) {

        // Check if the hare would still be on the game board after moving that distance
        if game_data.our_hare.position + distance > 44 {
            break; // All moves after this one will be outside the map
        }

        let move_carrot_price: u16 = triangular_number(distance as u16);

        // Check if our hare has enough carrots to move the distance
        if (game_data.our_hare.carrots as u16) < move_carrot_price {
            break; // All moves after this one will also be too expensive
        }

        // Check if the enemy hare is on the new field after the move
        if game_data.our_hare.position + distance == game_data.enemy_hare.position {
            continue;
        }

        // Get the field on which the hare would be after moving the current distance
        let new_field: FieldType = game_data.board.get_field((game_data.our_hare.position + distance) as usize).unwrap();

        // Match the type of the field
        match new_field {
            // If the field is a pos. 1, pos. 2 or carrots field is the move always possible
            FieldType::Position1 | FieldType::Position2 | FieldType::Carrots => {
                legal_moves.push(Box::new(AdvanceMove::new(distance, None)));
            },
            // If the field is a salad field does our hare has too have at least one salad
            FieldType::Salad => {
                if game_data.our_hare.salads > 0 {
                    legal_moves.push(Box::new(AdvanceMove::new(distance, None)));
                }
            },
            // If the field is a market field does our hare has too have at least an aditional 10 carrots
            FieldType::Market => {
                if game_data.our_hare.carrots >= move_carrot_price + 10 {
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(Card::EatSalad))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(Card::FallBack))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(Card::HurryAhead))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(Card::SwapCarrots))));
                }
            },
            // If the field is a hare field check if the hare has cards and if they are legal to be played
            FieldType::Hare => {
                for card in &game_data.our_hare.cards {
                    println!("{:?}", card_to_string(&card));

                    // TBD
                }
            },
            _ => {
                // If none of these match then print the current field type and continue to the next field
                println!("Unevaluated field: {:?}", new_field);
                continue;
            }
        }
    }

    legal_moves
}