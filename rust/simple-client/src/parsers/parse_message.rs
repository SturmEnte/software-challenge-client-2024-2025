use std::net::TcpStream;
use std::io::Write;

use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::structs::game_data::GameData;
use super::parse_welcome_message::parse_welcome_message;
use super::parse_memento::parse_memento;
use crate::computers::compute_move::compute_move;
use crate::structs::game_move::Move;

pub fn parse_message(buffer: [u8; 5000], n: usize, mut game_data: &mut GameData, stream: &mut TcpStream) {

    // Remove empty bytes from the buffer
    let message: &[u8] = &buffer[..n];
    // Turn the buffer into a string (message)
    let message_str: String = String::from_utf8(message.to_vec()).unwrap();

    // Create the XML reader
    let mut reader = Reader::from_str(&message_str);

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"data") => {
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"class")) {
                            // Retreive the class attribute's content as a computable string
                            let class = attr.unwrap().unescape_value().unwrap().to_string();
                        
                            // Execute the corresponding function based on the class attribute
                            match class.as_str() {
                                "welcomeMessage" => {
                                    if crate::DEBUGGING {
                                        println!("Welcome message");
                                    }
                                    // Parse the welcome message
                                    // This will set our own team and the opponent team in the game data
                                    parse_welcome_message(&e, &mut game_data);
                                },
                                "memento" => {
                                    if crate::DEBUGGING {
                                        println!("Memento");
                                    }
                                    parse_memento(&message_str, &mut game_data);
                                },
                                "moveRequest" => {
                                    if crate::DEBUGGING {
                                        println!("Move Request");
                                    }
                                    // The move that should be executed
                                    let m: Box<dyn Move> = compute_move(&game_data);

                                    // Create the move message string from the selected move
                                    let move_message = format!("<room roomId=\"{}\">{}</room>", game_data.room_id, m.to_string());
                                    
                                    // Print the move for debugging
                                    if crate::DEBUGGING {
                                        println!("{}", move_message);
                                    }

                                    // Save the move in the game data
                                    game_data.our_hare.last_move_type = Some(m.get_type());
                                    // game_data.our_hare.last_move = Some(m);

                                    // Send the move to the server
                                    _ = stream.write(move_message.as_bytes());
                                },
                                "result" => {
                                    if crate::DEBUGGING {
                                        println!("Result");
                                    }
                                    // Set the game to be done if a result is received
                                    game_data.game_over = true;
                                },
                                _ => {
                                    println!("Unknown class: {}", class);
                                },
                            }

                            // Break after finding the data type and processing it
                            break;
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