// use std::sync::Mutex;
use std::net::TcpStream;
// use std::io::Write;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::name::QName;

// use crate::parse_memento::parse_memento;
// use crate::GameData;
// use crate::compute::compute_move;
// use crate::Move;

use crate::board::Board;

pub fn parse_message(buffer: [u8; 5000], n: usize, /*game_data: &Mutex<GameData>,*/ stream: &mut Option<&mut TcpStream>) {

    // Remove empty bytes from the buffer
    let message: &[u8] = &buffer[..n];

    // Print the buffer as a string
    let message_str: String = String::from_utf8(message.to_vec()).unwrap();
    println!("{}", message_str);

    let mut reader = Reader::from_str(&message_str);
    // reader.trim_text(true);


    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"data") => {
                        if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"class")) {
                            let class = attr.unwrap().unescape_value().unwrap().to_string();
                            println!("Class attribute: {}", class);
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
//     let mut reader: Reader<&[u8]> = Reader::from_bytes(&message);
//     reader.trim_text(true);
//     reader.expand_empty_elements(true);

//     let mut buf: Vec<u8> = Vec::new();
    
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(ref e)) => {
//                 match e.name() {
//                     b"data" => {
//                         let class: String = String::from_utf8(e.try_get_attribute("class").unwrap().unwrap().value.to_vec()).unwrap();

//                         match class.as_str() {
//                             "welcomeMessage" => welcome_message(e, &game_data),
//                             "memento" => {
//                                 println!("Memento");
//                                 parse_memento(&message, &game_data);
//                             },
//                             "moveRequest" => {
//                                 println!("Move Request");
//                                 let mv: Move = compute_move(&game_data);
//                                 mv.print();
//                                 let send_str: String;
//                                 if mv.from_x == -1{
//                                     send_str = format!(r#"<room roomId="{}"><data class="move"><to x="{}" y="{}"/></data></room>"#, game_data.lock().unwrap().room_id, mv.to_x, mv.to_y);
//                                 }
//                                 else {
//                                     send_str = format!(r#"<room roomId="{}"><data class="move"><from x="{}" y="{}"/><to x="{}" y="{}"/></data></room>"#, game_data.lock().unwrap().room_id, mv.from_x, mv.from_y, mv.to_x, mv.to_y);
//                                 }
                                
//                                 stream.as_mut().unwrap().write(send_str.as_bytes()).unwrap();

//                             },
//                             "result" => {
//                                 println!("Result");
//                                 return true;
//                             },
//                             _ => (),
//                         }
//                     },
//                     _ => (),
//                 }
//             },
//             Ok(Event::Eof) => break,
//             Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//             _ => (), 
//         }
//         buf.clear();
//     }
//     false
}

// fn welcome_message(e: &BytesStart, data: &Mutex<GameData>) {
//     let team = String::from_utf8(e.try_get_attribute("color").unwrap().unwrap().value.to_vec()).unwrap();
//     println!("Received welcome message");
//     println!("Team: {}", &team);
//     data.lock().unwrap().set_team(&team);
// }