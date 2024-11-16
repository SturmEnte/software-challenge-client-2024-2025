use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::enums::team::Team;
use crate::structs::game_data::GameData;

pub fn parse_memento(message: &String, game_data: &mut GameData) {
     // Create the XML reader
     let mut reader = Reader::from_str(&message);

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
                        if game_data.start_team == Team::Undefined {
                            continue;
                        }

                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"startTeam")) {
                            let team: String = attr.unwrap().unescape_value().unwrap().parse().unwrap();
                            game_data.set_start_team(team.as_str());
                        }
                    },
                    QName(b"hare") => {
                        let mut team: Option<Team> = None;
                        let mut position: Option<i8> = None;
                        let mut salads: Option<i8> = None;
                        let mut carrots: Option<i8> = None;

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
                            if game_data.our_hare.team == team.clone().unwrap() {
                                game_data.our_hare.position = position.unwrap();
                                game_data.our_hare.salads = salads.unwrap();
                                game_data.our_hare.carrots = carrots.unwrap();
                            } else {
                                game_data.enemy_hare.team = team.clone().unwrap();
                                game_data.enemy_hare.position = position.unwrap();
                                game_data.enemy_hare.salads = salads.unwrap();
                                game_data.enemy_hare.carrots = carrots.unwrap();
                            }
                        } else {
                            println!("Hare: Missing attributes");
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
            Ok(Event::Eof) => break, // Exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}