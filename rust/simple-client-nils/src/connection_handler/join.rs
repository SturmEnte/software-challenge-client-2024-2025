use std::io::Write;

use xml::EventReader;

use crate::{error::ConnectionHandlerError, connection_handler::parsers::parse_joined::parse_joined};
use super::connection_handler::ConnectionHandler;
use crate::computer_player::ComputerPlayer;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    /// Attempts to join a game using an optional reservation code.
    ///
    /// # Arguments
    ///
    /// * `reservation_code` - An optional string slice that holds the reservation code for a game.
    ///
    /// # Example
    ///
    /// ```
    /// let mut connectionhandler = ConnectionHandler::new(player).unwrap();
    /// match connectionhandler.join((None)) {
    ///     Ok(()) => println!("Successfully joined the game room."),
    ///     Err(e) => eprintln!("Failed to join the game room: {:?}", e),
    /// }
    /// ```
    pub fn join(&mut self, reservation_code: Option<&str>) -> Result<(), ConnectionHandlerError> {
        let mut buffer = [0; 200];

        match reservation_code {
            Some(rc) => self.connection.write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", rc).as_bytes())?,
            None => self.connection.write(b"<protocol><join/>")?
        };

        let last_none_zero_index = self.read_to_buffer(&mut buffer)? -1;

        if buffer.is_empty() {return Err(ConnectionHandlerError::ZeroBytesReadToBuffer);}

        if !buffer.starts_with(b"<protocol>"){return Err(ConnectionHandlerError::MissingElement(String::from("protocol")));}

        let parser = EventReader::new(&buffer[10..=last_none_zero_index]);
        
        self.room_id = Some(parse_joined(parser)?);
        self.connected = true; 
        return Ok(());
    }
}