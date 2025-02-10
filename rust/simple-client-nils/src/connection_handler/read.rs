use std::io::Read;

use crate::error::ConnectionHandlerError;
use super::connection_handler::ConnectionHandler;
use crate::computer_player::ComputerPlayer;
use crate::utils::find_last_none_zero_index::find_last_non_zero_index;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(super) fn read_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ConnectionHandlerError> {
        match self.connection.read(buffer){
            Ok(0) => {
                return Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => return Ok(b),
            Err(e) => return Err(ConnectionHandlerError::Io(e)),
        }
    }

    pub(super) fn read_full_message_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ConnectionHandlerError> {
        self.read_to_buffer(buffer)?;

        let mut last_non_zero_index = find_last_non_zero_index(&buffer);

        if buffer.starts_with(b"<left") || buffer.starts_with(b"<protocol>") || buffer[..=last_non_zero_index].ends_with(b"</room>") { return Ok(last_non_zero_index) };

        loop{
            let number_of_new_bytes = self.read_to_buffer(&mut buffer[last_non_zero_index + 1..])?;
            last_non_zero_index += number_of_new_bytes;
            if buffer[..=last_non_zero_index].ends_with(b"</room>") {return Ok(last_non_zero_index)};
        } 
    }
}