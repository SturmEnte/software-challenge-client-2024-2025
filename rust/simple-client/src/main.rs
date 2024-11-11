mod utils;
mod parse_message;
mod board;
mod field_type;

use std::fs::{File, remove_file};
use std::net::TcpStream;
use std::io::{Write, Read, Cursor};

use utils::get_cmd_args::get_join_info;
use parse_message::parse_message;

fn main() {
    let join_info: (String, String) = get_join_info();
    let server_address: &str = join_info.0.as_str();
    let join_msg: &str = join_info.1.as_str();
     
    let mut global_buffer: Cursor<[u8; 5000]> = Cursor::new([0; 5000]);
    let mut global_n: usize = 0usize;
    let mut _msg: i32 = 0;

    let mut stream = TcpStream::connect(server_address).unwrap();

    // Send join message
    stream.write(join_msg.as_bytes()).unwrap();

    // Delete old and create new buffer file for debugging
    match remove_file("buffer.xml") {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }
    let mut file = File::create("buffer.xml").unwrap();

    // Continuesly read messages from the server and react to them
    loop {
        let mut buffer: [u8; 5000] = [0; 5000];
        let n: usize = stream.read(&mut buffer[..]).unwrap();
        
        // Write buffer to file for debugging
        file.write(&buffer[..n]).unwrap();

        if buffer.starts_with(b"<protocol>") { // executes at the beginning of the communication to rertrieve the room id
            println!("Joined room");
            //game_data.lock().unwrap().room_id = get_room_id(&buffer);
            //println!("Room id: {}", game_data.lock().unwrap().room_id);
            continue;
        } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() { // executes if a new room tag is closed
            // Add new data to the global buffer
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;

            parse_message(global_buffer.into_inner(), global_n, &mut Some(&mut stream));

            // let game_end: bool = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut Some(&mut stream));

            // if game_end {
            //     break;
            // }

            // Reset the global buffer after a room tag was processed
            global_buffer = Cursor::new([0; 5000]);
            global_n = 0usize;
            
            continue;
        } 

        // Add the buffer data to the global buffer if no room tag was closed
        global_buffer.write(&buffer[..n]).unwrap();
        global_n += n;
    }
}
