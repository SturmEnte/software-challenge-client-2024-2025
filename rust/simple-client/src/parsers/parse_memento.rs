use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::enums::move_type::MoveType;
use crate::enums::team::Team;
use crate::structs::game_data::GameData;

use crate::structs::game_move::Move;
use crate::structs::game_move::AdvanceMove;

use crate::enums::card::string_to_card;

pub fn parse_memento(message: &String, game_data: &mut GameData) {
     // Create the XML reader
     let mut reader = Reader::from_str(&message);

    let mut current_team: Option<Team> = None;  // The team of the hare that is currently being parsed 
    // let mut hares_cards = false;
    // let mut inside_card = false;

    // let mut last_text: String = String::new();

     loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"state") => {
                        // Retreive the turn
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"turn")) {
                            let turn: i8 = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            game_data.turn = turn;
                        }

                        // Retreive the starting team if it is not defined yet
                        if game_data.start_team.is_some() {
                            continue;
                        }

                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"startTeam")) {
                            let team: String = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            game_data.set_start_team(team.as_str());
                        }
                    },
                    QName(b"hare") => {
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

                            current_team = team.clone();
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

                        // Iterate over the child elements of the board
                        let mut cards = false;

                        loop {
                            match reader.read_event() {
                                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                                    
                                    if e.name() == QName(b"cards") {
                                        cards = true;
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
                        }
                    },
                    // QName(b"cards") => {
                    //     hares_cards = true;
                    // },
                    // QName(b"card") => {
                    //     inside_card = true;

                    //     //println!("Card: {}", e.unescape().unwrap().into_owned());
                    //     println!("Card: {}", &last_text);

                    //     //let card: String = e.unescape().unwrap().into_owned();
                        
                    //     if currentTeam == game_data.our_hare.team {
                    //         game_data.our_hare.cards.push(string_to_card(&last_text));
                    //     } else {
                    //         game_data.enemy_hare.cards.push(string_to_card(&last_text));
                    //     }
                    // },
                    QName(b"lastAction") => {
                        // hares_cards = false;

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
                                // game_data.our_hare.last_move = last_move;
                                // game_data.our_hare.last_move_type = last_move_type;
                            } else {
                                game_data.enemy_hare.last_move = last_move;
                                game_data.enemy_hare.last_move_type = last_move_type;
                            }
                        }
                    },
                    QName(b"board") => {
                        // Only parse the board, if it is not initialized yet
                        if game_data.board.initialized {
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
                                            println!("Field {}: {}", i, field_text);
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

                        game_data.board.initialized = true;
                        game_data.board.print();
                    },
                    _ => (),
                }
            },
            // Ok(Event::Text(e)) => {
            //     last_text = e.unescape().unwrap().into_owned();
            //     /*if inside_card && hares_cards {

            //         println!("Card: {}", e.unescape().unwrap().into_owned());

            //         let card: String = e.unescape().unwrap().into_owned();
                    
            //         if currentTeam == game_data.our_hare.team {
            //             game_data.our_hare.cards.push(string_to_card(&card));
            //         } else {
            //             game_data.enemy_hare.cards.push(string_to_card(&card));
            //         }

            //         inside_card = false;
            //     }*/
            // }
            Ok(Event::Eof) => break, // Exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}