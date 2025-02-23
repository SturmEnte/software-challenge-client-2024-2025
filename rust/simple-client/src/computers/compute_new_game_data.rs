use colored::Colorize;

// Fix this
// v 
use crate::{enums::{field_type::FieldType, move_type::MoveType}, structs::{game_data::GameData, game_move::{AdvanceMove, ExchangeCarrotsMove, Move}, hare::Hare}, utils::{get_nearest_hedgehog_field::get_nearest_hedgehog_field, triangular_number::triangular_number}};

use crate::enums::card::Card;

pub fn compute_new_game_data(game_data: &GameData, m: &Box<dyn Move>, our_hares_move: &bool) -> GameData {
    let mut new_game_data: GameData = game_data.clone();

    let mut current_hare: Hare = if *our_hares_move { new_game_data.our_hare.clone() } else { new_game_data.enemy_hare.clone() };
    let mut other_hare: Hare = if *our_hares_move { new_game_data.enemy_hare.clone() } else { new_game_data.our_hare.clone() };

    // If the hare is on a position 1 field and it is the first hare, then it will get 10 carrots  
    if game_data.board.get_field(current_hare.position.into()).unwrap() == FieldType::Position1 && current_hare.position > other_hare.position {
        current_hare.carrots += 10;
    }

    // If the hare is on a position 2 field and it is the second hare, then it will get 30 carrots  
    if game_data.board.get_field(current_hare.position.into()).unwrap() == FieldType::Position2 && current_hare.position < other_hare.position {
        current_hare.carrots += 30;
    }

    // Simulate different move types
    match m.get_type() {
        MoveType::Advance => {
            let advance_move = m.as_any().downcast_ref::<AdvanceMove>().unwrap();
            
            current_hare.position += advance_move.distance;
            current_hare.carrots -= triangular_number(advance_move.distance.into());

            if advance_move.cards.is_some() {
                
                for card in advance_move.cards.as_ref().unwrap() {
                    
                    // Check if the card is for a market field purchase
                    // If so subtract 10 carrots from the hare and add the card to the hare's cards
                    if game_data.board.get_field(current_hare.position.into()).unwrap() == FieldType::Market {
                        current_hare.carrots -= 10;
                        current_hare.cards.push(card.clone());
                        continue;
                    }
                    
                    // Otehrwise match the card an simulate the execution of the card
                    match card {
                        Card::EatSalad => {
                            current_hare.salads -= 1;
                            
                            if current_hare.position > other_hare.position {
                                current_hare.carrots += 10;
                            } else {
                                current_hare.carrots += 30;
                            }
                        },
                        Card::FallBack => {
                            current_hare.position = other_hare.position - 1;
                        },
                        Card::HurryAhead => {
                            current_hare.position = other_hare.position + 1;
                        },
                        Card::SwapCarrots => {
                            let temp = current_hare.carrots;
                            current_hare.carrots = other_hare.carrots;
                            other_hare.carrots = temp;
                        }
                    }
                }    
            }

            println!("Advance Move | Distance: {} | Cost: {}", advance_move.distance, triangular_number(advance_move.distance.into()));

            // Subtract 10 carrots if the hare is on a market field after the advance move
            if game_data.board.get_field(current_hare.position.into()).unwrap() == FieldType::Market {
                current_hare.carrots -= 10;
                println!("Market Field | Cost: 10");
                println!("{}", "Card is not read".red());
            }

            if advance_move.cards.is_some() {
                println!("{}", "Don't know how to simulate a card".red());
            }
        },
        MoveType::EatSalad => {
            current_hare.salads -= 1;
            
            // If the hare is first it gets 10 carrots else it gets 30 carrots
            if current_hare.position > other_hare.position {
                current_hare.carrots += 10;
            } else {
                current_hare.carrots += 30;
            }
        },
        MoveType::ExchangeCarrots => {
            let exchange_carrots_move = m.as_any().downcast_ref::<ExchangeCarrotsMove>().unwrap();
            current_hare.carrots = ((current_hare.carrots as i16 + exchange_carrots_move.amount as i16)).try_into().unwrap();
        },
        MoveType::Fallback => {
            // Unwrap is safe to use here, because there must be a hedgehog field behind the current hare, otherwise there is something weird going on
            let nearest_hedghehog_field = get_nearest_hedgehog_field(&current_hare.position).unwrap();
            
            // The hare gets 10 carrots for each field it moves back
            current_hare.carrots += (current_hare.position as u16 - *nearest_hedghehog_field as u16) * 10;

            println!("Gained carrots because of fallback: {}" ,(current_hare.position as u16 - *nearest_hedghehog_field as u16) * 10);

            current_hare.position = *nearest_hedghehog_field;
        }
    }

    println!("---------");
    current_hare.print();
    println!("---------");
    other_hare.print();
    println!("---------");

    if *our_hares_move {
        new_game_data.our_hare = current_hare;
        new_game_data.enemy_hare = other_hare;
    } else {
        new_game_data.our_hare = other_hare;
        new_game_data.enemy_hare = current_hare;
    }

    new_game_data
}