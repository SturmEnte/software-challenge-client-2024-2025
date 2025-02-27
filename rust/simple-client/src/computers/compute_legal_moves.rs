use colored::Colorize;

use crate::enums::card::card_to_string;
use crate::enums::field_type::FieldType;
use crate::enums::move_type::MoveType;

use crate::structs::board::Board;
use crate::structs::hare::Hare;
use crate::utils::triangular_number::triangular_number;
use crate::GameData;

use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;
use crate::structs::game_move::FallbackMove;
use crate::structs::game_move::EatSaladMove;
use crate::structs::game_move::ExchangeCarrotsMove;

use crate::enums::card::Card;

const LAST_SALAD_FIELD: u8 = 57;

pub fn compute_legal_moves(game_data: &GameData) -> Vec<Box<dyn Move>> {
    let mut legal_moves: Vec<Box<dyn Move>> = Vec::new();

    // Eat salad move
    // Check if the last move was an advance move and if the hare is on a salad field
    // If so, is the hare forced to eat a salad
    if  game_data.board.board[game_data.our_hare.position as usize].unwrap() == FieldType::Salad    // Check if the current field is a salad field
        && game_data.our_hare.last_move_type.is_some()                                              // Check if the hare has a last move
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
        if (game_data.our_hare.carrots as u16) <= move_carrot_price {
            break; // All moves after this one will also be too expensive
        }

        // Check if the enemy hare is on the new field after the move and the new field is not the goal
        if game_data.our_hare.position + distance == game_data.enemy_hare.position && game_data.board.get_field(&((game_data.our_hare.position + distance) as usize)).unwrap() != FieldType::Goal {
            continue;
        }

        // Get the field on which the hare would be after moving the current distance
        let new_field: FieldType = game_data.board.get_field(&((game_data.our_hare.position + distance) as usize)).unwrap();

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
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::EatSalad]))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::FallBack]))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::HurryAhead]))));
                    legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::SwapCarrots]))));
                }
            },
            // If the field is a hare field check if the hare has cards and if they are legal to be played
            FieldType::Hare => {
                for card in &game_data.our_hare.cards {
                    println!("{:?}", card_to_string(&card).as_str().blue());

                    match card {
                        Card::EatSalad => {
                            if game_data.our_hare.salads > 0 {
                                legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::EatSalad]))));
                            }
                        },
                        Card::SwapCarrots => {
                            if game_data.last_swap_carrots_usage + 2 < game_data.turn as i8 && game_data.our_hare.position < LAST_SALAD_FIELD && game_data.enemy_hare.position < LAST_SALAD_FIELD {
                                legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::SwapCarrots]))));
                            }
                        },
                        Card::FallBack => {
                            if game_data.our_hare.position > game_data.enemy_hare.position && 
                               is_allowed_to_go_on_field(&game_data.our_hare, &game_data.enemy_hare.position, &(game_data.our_hare.position - 1), &game_data.board) {
                                legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::FallBack]))));
                            }

                            // Compute complicated fallback
                        },
                        Card::HurryAhead => {
                            if game_data.our_hare.position < game_data.enemy_hare.position && 
                               is_allowed_to_go_on_field(&game_data.our_hare, &game_data.enemy_hare.position, &(game_data.our_hare.position + 1), &game_data.board) {
                                legal_moves.push(Box::new(AdvanceMove::new(distance, Some(vec![Card::HurryAhead]))));
                            }

                            // Compute complicated hurry ahead
                        }
                    }
                }
            },
            // If the field is a goal field check if our hare has at most 10 carrots and no salads
            FieldType::Goal => {
                if game_data.our_hare.carrots <= 10 && game_data.our_hare.salads == 0 {
                    legal_moves.push(Box::new(AdvanceMove::new(distance, None)));
                }
            }
            _ => {
                // If none of these match then print the current field type and continue to the next field
                println!("Unevaluated field: {:?}", new_field);
                continue;
            }
        }
    }

    legal_moves
}

// Returns if a hare is allowed on a curtain field (ignoring hare fields)
fn is_allowed_to_go_on_field(hare: &Hare, other_hares_position: &u8, position: &u8, board: &Board) -> bool {

    let new_field: FieldType = board.get_field(&(*position as usize)).unwrap();

    if position == other_hares_position && new_field != FieldType::Goal {
        return false;
    }

    let mut allowed: bool = false;

    match new_field {
        // If the field is a pos. 1, pos. 2 or carrots field is the move always possible
        FieldType::Position1 | FieldType::Position2 | FieldType::Carrots => {
            allowed = true;
        },
        // If the field is a salad field does our hare has too have at least one salad
        FieldType::Salad => {
            if hare.salads > 0 {
                allowed = true;
            }
        },
        // If the field is a market field does our hare has too have at least an aditional 10 carrots
        FieldType::Market => {
            if hare.carrots >= 10 {
                allowed = true;
            }
        },
        // If the field is a goal field check if our hare has at most 10 carrots and no salads
        FieldType::Goal => {
            if hare.carrots <= 10 && hare.salads == 0 {
                allowed = true;
            }
        }
        _ => {
            // You are not allowed to go on a start field after leaving it
            // You can only go on a hedgehog field with a fallback move not the fallback card
            // Hare fields are ignored in this function
            allowed = false; // This should be unnecicary but it isnt for some reason I dont know yet
        }
    }
    
    allowed
}

#[cfg(test)]
mod tests {
    use crate::{enums::{card::Card, field_type::FieldType, move_type::MoveType, team::Team}, structs::{board::Board, game_data::GameData, game_move::{AdvanceMove, Move}, hare::Hare}};

    use super::compute_legal_moves;

    #[test]
    fn test_compute_legal_moves() {
        let mut game_data = GameData::new();

        game_data.turn = 1;
        game_data.start_team = Some(Team::One);

        game_data.board = Board::new();
        game_data.board.board = [
            Some(FieldType::Start),
            Some(FieldType::Market),
            Some(FieldType::Position1),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Position2),
            Some(FieldType::Salad),
            Some(FieldType::Hedgehog),
            Some(FieldType::Carrots),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Hedgehog),
            Some(FieldType::Position2),
            Some(FieldType::Market),
            Some(FieldType::Position1),
            Some(FieldType::Hedgehog),
            Some(FieldType::Carrots),
            Some(FieldType::Carrots),
            Some(FieldType::Position2),
            Some(FieldType::Salad),
            Some(FieldType::Hedgehog),
            Some(FieldType::Market),
            Some(FieldType::Hare),
            Some(FieldType::Position2),
            Some(FieldType::Carrots),
            Some(FieldType::Carrots),
            Some(FieldType::Hedgehog),
            Some(FieldType::Position2),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Position1),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Hedgehog),
            Some(FieldType::Carrots),
            Some(FieldType::Position2),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Salad),
            Some(FieldType::Hedgehog),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Position1),
            Some(FieldType::Position2),
            Some(FieldType::Market),
            Some(FieldType::Carrots),
            Some(FieldType::Hedgehog),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Market),
            Some(FieldType::Position2),
            Some(FieldType::Carrots),
            Some(FieldType::Hedgehog),
            Some(FieldType::Salad),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Position1),
            Some(FieldType::Carrots),
            Some(FieldType::Hare),
            Some(FieldType::Carrots),
            Some(FieldType::Goal)
        ];

        game_data.our_hare = Hare::new();

        game_data.enemy_hare = Hare::new();

        let legal_moves = compute_legal_moves(&game_data);

        let mut i = 1;

        for m in legal_moves {
            print!("{} {:?}", i, m.get_type());

            if m.get_type() == MoveType::Advance {
                let advance_move = m.as_any().downcast_ref::<AdvanceMove>().unwrap();
                
                print!(" {} ", advance_move.distance);

                if advance_move.cards.is_some() {

                    print!("with cards: ");

                    for card in advance_move.cards.as_ref().unwrap() {
                        print!("{:?} ", card);
                    }
                    
                }

                println!();
            }

            i += 1;
        }

        let expected_moves: Vec<Box<dyn Move>> = vec![
            Box::new(AdvanceMove::new(1, Some(vec![Card::EatSalad]))),
            Box::new(AdvanceMove::new(1, Some(vec![Card::FallBack]))),
            Box::new(AdvanceMove::new(1, Some(vec![Card::HurryAhead]))),
            Box::new(AdvanceMove::new(1, Some(vec![Card::SwapCarrots]))),
            Box::new(AdvanceMove::new(2, None)),
            Box::new(AdvanceMove::new(2, None)),
            Box::new(AdvanceMove::new(6, None)),
            Box::new(AdvanceMove::new(7, None)),
            Box::new(AdvanceMove::new(9, None)),
            Box::new(AdvanceMove::new(10, None)),
        ];

        let equal = true;

        // Check if all moves that are in the valid moves vector are also inside the excpected moves vector
        // If there are missing moves, then set missing_moves to true

        assert_eq!(equal, true);
    }
}