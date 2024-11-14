use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::enums::team::Team;
use crate::structs::game_data::GameData;

pub fn parse_memento(message: &String, mut game_data: &mut GameData) {
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
                            // println!("Turn: {}", turn);
                            game_data.turn = turn;
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
                    _ => (),
                }
            },
            Ok(Event::Eof) => break, // Exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}