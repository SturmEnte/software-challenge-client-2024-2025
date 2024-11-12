use std::net::TcpStream;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::name::QName;

use crate::game_data::GameData;

// use crate::board::Board;

pub fn parse_message(buffer: [u8; 5000], n: usize, mut game_data: &mut GameData, stream: &mut Option<&mut TcpStream>) {

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
                                },
                                "memento" => {
                                    println!("Memento");
                                },
                                "moveRequest" => {
                                    println!("Move Request");
                                },
                                "result" => {
                                    println!("Result");
                                },
                                _ => {
                                    println!("Unknown class: {}", class);
                                },
                            }
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