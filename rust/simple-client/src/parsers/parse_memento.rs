use colored::Colorize;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::computers::compute_new_game_data::compute_new_game_data;
use crate::enums::move_type::MoveType;
use crate::enums::team::Team;
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
                            let turn: i8 = attr.unwrap().unescape_value().unwrap().parse().unwrap();
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
                                        
                                        // TBD Check for cards

                                        last_move = Some(Box::new(AdvanceMove::new(distance, None)));
                                    }
                                },
                                "eatsalad" => {
                                    last_move = Some(Box::new(EatSaladMove::new()));
                                },
                                "fallback" => {
                                    last_move = Some(Box::new(FallbackMove::new()));
                                },
                                "exchangecarrots" => {
                                    //last_move = Some(Box::new(ExchangeCarrotsMove::new()));
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
                    /*QName(b"hare") => {

                        // Only read hare data if it's turn 0
                        if game_data.turn > 0 {
                            continue;
                        }

                        let mut team: Option<Team> = None;
                        let mut position: Option<u8> = None;
                        let mut salads: Option<u8> = None;
                        let mut carrots: Option<u16> = None;

                        // Retreive the team
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"team")) {
                            let attr_team = attr.unwrap().unescape_value().unwrap().to_string();

                            if attr_team == "ONE" {
                                team = Some(Team::One);
                            } else {
                                team = Some(Team::Two);
                            }
                        }

                        // Retreive the position
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"position")) {
                            position = Some(attr.unwrap().unescape_value().unwrap().parse().unwrap());
                        }

                        // Retreive the salads
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"salads")) {
                            salads = Some(attr.unwrap().unescape_value().unwrap().parse().unwrap());
                        }

                        // Retreive the carrots
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"carrots")) {
                            carrots = Some(attr.unwrap().unescape_value().unwrap().parse().unwrap());
                        }

                        // Set the hare's attributes in the game data if all attributes were successfully retreived
                        if team.is_some() && position.is_some() && salads.is_some() && carrots.is_some() {
                            // println!("Hare: Team: {:?}, Position: {}, Salads: {}, Carrots: {}", team.clone().unwrap(), position.unwrap(), salads.unwrap(), carrots.unwrap()); 
                            if game_data.our_hare.team.clone().unwrap().clone() == team.clone().unwrap() {
                                game_data.our_hare.position = position.unwrap();
                                game_data.our_hare.salads = salads.unwrap();
                                game_data.our_hare.carrots = carrots.unwrap();
                            } else {
                                game_data.enemy_hare.team = Some(team.clone().unwrap());
                                game_data.enemy_hare.position = position.unwrap();
                                game_data.enemy_hare.salads = salads.unwrap();
                                game_data.enemy_hare.carrots = carrots.unwrap();
                            }
                        } else {
                            println!("Hare: Missing attributes");
                        }

                        println!("{}{:?}{}", "Read hare data (Hare ".green(), team.unwrap(), ")".green());

                        /* 
                        // Iterate over the child elements of the board
                        let mut cards = false;

                        loop {
                            match reader.read_event() {
                                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                                    
                                    if e.name() == QName(b"cards") {
                                        cards = true;

                                        // Reset the card array of the current hare
                                        if current_team == game_data.our_hare.team {
                                            game_data.our_hare.cards = Vec::new();
                                        } else {
                                            game_data.enemy_hare.cards = Vec::new();
                                        }
                                        
                                    } else if e.name() == QName(b"cards") {
                                        cards = false;
                                    } else if e.name() == QName(b"card") {
                                        // Retrieve the text content of the card
                                        if let Ok(Event::Text(e)) = reader.read_event() {
                                            let card_text: String = e.unescape().unwrap().into_owned();
                                            
                                            if cards == true {
                                                if current_team == game_data.our_hare.team {
                                                    game_data.our_hare.cards.push(string_to_card(&card_text));
                                                } else {
                                                    game_data.enemy_hare.cards.push(string_to_card(&card_text));
                                                }
                                            }
                                        }
                                    }
                                },
                                Ok(Event::End(ref e)) if e.name() == QName(b"hare") => {
                                    break;
                                },
                                Ok(Event::Eof) => {
                                    break;
                                },
                                _ => (),
                            }
                        } */
                    },*/
                    /*
                    QName(b"lastAction") => {
                        // Retreiive the class of the last action
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"class")) {
                            let class: String = attr.unwrap().unescape_value().unwrap().to_string();
                            let distance: Option<u8> = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"distance")).map(|a| a.unwrap().unescape_value().unwrap().parse().unwrap());

                            let last_move: Option<Box<dyn Move>>;
                            let last_move_type: Option<MoveType>;

                            match class.as_str() {
                                "advance" => {
                                    last_move = Some(Box::new(AdvanceMove::new(distance.unwrap(), None)));
                                    last_move_type = Some(MoveType::Advance);
                                },
                                _ => {
                                    println!("Unknown last action class: {}", class);
                                    last_move = None;
                                    last_move_type = None;
                                }
                            }
                            
                            if current_team == game_data.our_hare.team {
                                game_data.enemy_hare.last_move = last_move;
                                game_data.enemy_hare.last_move_type = last_move_type;
                            }
                        }
                    },
                    */
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