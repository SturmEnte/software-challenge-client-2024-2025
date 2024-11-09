mod utils;

use std::net::TcpStream;
use std::io::{Write, Read, Cursor};

use utils::get_cmd_args::get_join_info;

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

    let mut file = File::create("buffer.txt").unwrap();

    // Continuesly read messages from the server and react to them
    loop {
        let mut buffer: [u8; 5000] = [0; 5000];
        let n: usize = stream.read(&mut buffer[..]).unwrap();
        
        // Write buffer to file for debugging
        
        file.write(&buffer[..n]).unwrap();

        // if buffer.starts_with(b"<protocol>") {
        //     println!("Joined room");
        //     //game_data.lock().unwrap().room_id = get_room_id(&buffer);
        //     //println!("Room id: {}", game_data.lock().unwrap().room_id);
        //     continue;
        // } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() { // returns true, if the data in the buffer ends with </room>
        //     global_buffer.write(&buffer[..n]).unwrap();
        //     global_n += n;

        //     /*let game_end: bool = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut Some(&mut stream));

        //     if game_end {
        //         break;
        //     }*/

        //     global_buffer = Cursor::new([0; 5000]);
        //     global_n = 0usize;
            
        //     continue;
        // } 

        // Add buffer data to the global buffer and add n to the global n
        global_buffer.write(&buffer[..n]).unwrap();
        global_n += n;
    }
}
