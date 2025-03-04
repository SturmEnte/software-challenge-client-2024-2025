use xml::EventReader;

use crate::error::ConnectionHandlerError;
use super::connection_handler::ConnectionHandler;
use crate::computer_player::ComputerPlayer;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    /// Enters the main game loop, processing messages from the server.
    ///
    /// # Notes
    ///
    /// This method will block and run indefinitely until the connection is closed or an error
    /// occurs. It is expected that the `ConnectionHandler` has joined a game using `join()`,
    pub fn play(&mut self) -> Result<(), ConnectionHandlerError> {
        while self.is_connected() {
            let mut buffer = [0; 3500];
            let last_none_zero_index = self.read_full_message_to_buffer(&mut buffer)?;

            let parser = EventReader::new(&buffer[..=last_none_zero_index]);

            match self.parse_message(parser) {
                Err(error) => eprintln!("{}", error),
                Ok(_) => {},
            }

        }
        return Ok(());
    }
}