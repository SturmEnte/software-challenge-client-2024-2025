use colored::Colorize;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::computers::compute_new_game_data::compute_new_game_data;

use crate::enums::card::Card;

use crate::structs::game_data::GameData;
use crate::structs::game_move::EatSaladMove;
use crate::structs::game_move::ExchangeCarrotsMove;
use crate::structs::game_move::FallbackMove;
use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;

use crate::enums::card::string_to_card;

pub fn parse_memento(message: &String, game_data: &mut GameData) {
     // Create the XML reader
     let mut reader = Reader::from_str(&message);

    // let mut current_team: Option<Team> = None;  // The team of the hare that is currently being parsed 

     loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"state") => {
                        // Retreive the turn
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"turn")) {
                            let turn: u8 = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            if crate::DEBUGGING {
                                println!("{}{}", "Turn: ".on_blue(), turn.to_string().on_yellow());
                            }
                            game_data.turn = turn;
                        }

                        // Continue if it's not turn 0
                        if game_data.turn != 0 {
                            continue;
                        }

                        // Retreive the starting team
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"startTeam")) {
                            let team: String = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            game_data.set_start_team(team.as_str());
                        }
                    },
                    QName(b"lastMove") => {
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"class")) {
                            let class: String = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            
                            if crate::DEBUGGING {
                                println!("{}{}", "Last Move Class: ".green(), class.to_string().cyan());
                            }
                            
                            let mut last_move: Option<Box<dyn Move>> = None;

                            match class.as_str() {
                                "advance" => {
                                    if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"distance")) {
                                        let distance: u8 = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                                        let mut cards: Vec<Card> = Vec::new();
                                
                                        // Read nested card elements
                                        loop {
                                            match reader.read_event() {
                                                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                                                    if e.name() == QName(b"card") {
                                                        if let Ok(Event::Text(e)) = reader.read_event() {
                                                            let card_text: String = e.unescape().unwrap().into_owned();
                                                            cards.push(string_to_card(&card_text));
                                                        }
                                                    }
                                                },
                                                Ok(Event::End(ref e)) if e.name() == QName(b"lastMove") => {
                                                    break;
                                                },
                                                Ok(Event::Eof) => {
                                                    break;
                                                },
                                                _ => (),
                                            }
                                        }
                                
                                        if cards.len() > 0 {
                                            last_move = Some(Box::new(AdvanceMove::new(distance, Some(cards))));
                                        } else {
                                            last_move = Some(Box::new(AdvanceMove::new(distance, None)));
                                        }
                                    }
                                },
                                "eatsalad" => {
                                    last_move = Some(Box::new(EatSaladMove::new()));
                                },
                                "fallback" => {
                                    last_move = Some(Box::new(FallbackMove::new()));
                                },
                                "exchangecarrots" => {
                                    if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"amount")) {
                                        let amount: i8 = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                                        
                                        // TBD Check for cards

                                        last_move = Some(Box::new(ExchangeCarrotsMove::new(amount)));
                                    }
                                },
                                _ => {
                                    println!("{}{}", "Unknown last move class: ".red(), class);
                                }
                            }

                            // This does not state if it's our turn but if the last move given in the memento is our hare's move
                            let mut our_hares_move: bool = false;

                            // If our team starts and it's an odd turn, then it's our hare's move
                            if game_data.our_hare.team == game_data.start_team && game_data.turn % 2 != 0 {         
                                our_hares_move = true;
                            // If our team doesn't start and it's an even turn, then it's our hare's move
                            } else if game_data.our_hare.team != game_data.start_team && game_data.turn % 2 == 0 {
                                our_hares_move = true;
                            }

                            if last_move.is_none() {
                                println!("{}", "No last move\nThis check is to be removed later if all moves are implemented".red());
                                continue;
                            }

                            // Compute the new game data based on the last move and store it in the game data
                            let new_game_data: GameData = compute_new_game_data(&game_data, &(last_move.unwrap()), &our_hares_move);
                            *game_data = new_game_data;
                        }
                    },
                    QName(b"board") => {
                        // Only parse the board on turn 0
                        if game_data.turn > 0 {
                            continue;
                        }
                        
                        // Iterator variable                        
                        let mut i: usize = 0;

                        // Iterate over the child elements of the board
                        loop {
                            match reader.read_event() {
                                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                                    if e.name() == QName(b"field") {
                                        // Retrieve the text content of the field
                                        if let Ok(Event::Text(e)) = reader.read_event() {
                                            let field_text: String = e.unescape().unwrap().into_owned();
                                            game_data.board.set_field(i, field_text.as_str());
                                            i += 1;
                                        }
                                    }
                                },
                                Ok(Event::End(ref e)) if e.name() == QName(b"board") => {
                                    break;
                                },
                                Ok(Event::Eof) => {
                                    break;
                                },
                                _ => (),
                            }
                        }

                        if crate::DEBUGGING {
                            game_data.board.print();
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break, // Exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}