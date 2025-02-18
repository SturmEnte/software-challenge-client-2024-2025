use colored::Colorize;

use crate::{enums::{field_type::FieldType, move_type::MoveType}, structs::{game_data::GameData, game_move::{AdvanceMove, Move}, hare::Hare}, utils::triangular_number::triangular_number};

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

            println!("Advance Move | Distance: {} | Cost: {}", advance_move.distance, triangular_number(advance_move.distance.into()));

            // Subtract 10 carrots if the hare is on a market field after the advance move
            if game_data.board.get_field(current_hare.position.into()).unwrap() == FieldType::Market {
                current_hare.carrots -= 10;
                println!("Market Field | Cost: 10");
                println!("{}", "Card is not read".red());
            }

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