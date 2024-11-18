use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::name::QName;

pub fn get_room_id(buffer: &[u8; 5000], n: &usize) -> String{
    
    // Remove empty bytes from the buffer
    let message: &[u8] = &buffer[..*n];
    // Turn the buffer into a string (message)
    let message_str: String = String::from_utf8(message.to_vec()).unwrap();

    let mut reader = Reader::from_str(&message_str);

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"joined") => {
                    println!("Joined");
                    // Retreive the roomId attribute from the joined tag
                    if let Some(attr) = e.attributes().find(|a| a.as_ref().unwrap().key == QName(b"roomId")) {
                        // Set the room id in the game data
                        return attr.unwrap().unescape_value().unwrap().to_string();
                    } else {
                        panic!("No roomId attribute found in joined tag");
                    }
                },
                    _ => (),
                }
            },
            // Ok(Event::Eof) => break, // Exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}