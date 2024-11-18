use std::net::TcpStream;
use std::io::Write;

use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;

use crate::structs::game_data::GameData;
use super::parse_welcome_message::parse_welcome_message;
use super::parse_memento::parse_memento;
use crate::computers::compute_legal_moves::compute_legal_moves;
use crate::structs::game_move::Move;

pub fn parse_message(buffer: [u8; 5000], n: usize, mut game_data: &mut GameData, stream: &mut TcpStream) {

    // Remove empty bytes from the buffer
    let message: &[u8] = &buffer[..n];
    // Turn the buffer into a string (message)
    let message_str: String = String::from_utf8(message.to_vec()).unwrap();

    if crate::DEBUGGING {
        // Print the buffer as a string
        println!("{}", message_str);
    }

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
                                    println!("Welcome message");
                                    // Parse the welcome message
                                    // This will set our own team and the opponent team in the game data
                                    parse_welcome_message(&e, &mut game_data);
                                },
                                "memento" => {
                                    println!("Memento");
                                    parse_memento(&message_str, &mut game_data);
                                },
                                "moveRequest" => {
                                    println!("Move Request");
                                    let moves = compute_legal_moves(&game_data);

                                    // Select a random valid move
                                    use rand::Rng;
                                    let mut rng = rand::thread_rng();
                                    let random_number: u32 = rng.gen_range(0..moves.len() as u32);

                                    let random_move: &dyn Move = &*moves[random_number as usize];

                                    // let mut actions: String = String::new();

                                    // let mut i: i8 = 0;
                                    // for action in &random_move.actions {
                                    //     actions.push_str(action.to_string(&i).as_str());
                                    //     i += 1;
                                    // }

                                    for m in &moves {
                                        println!("{}", m.to_string());
                                    }

                                    let move_message = format!("<room roomId=\"{}\">{}</room>", game_data.room_id, random_move.to_string()); //<data class=\"move\"></data>
                                    println!("Move: {}", move_message);

                                    _ = stream.write(move_message.as_bytes());
                                },
                                "result" => {
                                    println!("Result");
                                    // Set the game to be done if a result is received
                                    game_data.game_over = true;
                                },
                                _ => {
                                    println!("Unknown class: {}", class);
                                },
                            }

                            // Break after finding the data type and processing it
                            break; // Remider to myself: This could cause problems but I dont think it will
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