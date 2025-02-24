use xml::{attribute::OwnedAttribute, EventReader};

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, utils::get_attribute::get_attribute};
use crate::error::ConnectionHandlerError;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(super) fn parse_data(&mut self, parser: EventReader<&[u8]>, attributes: &Vec<OwnedAttribute>) -> Result<(), ConnectionHandlerError> {
        match get_attribute(attributes, "class")?.as_str() {
            "welcomeMessage" => {
                self.parse_welcomemessage(attributes)?;
            },
            "memento" => {
                self.parse_memento(parser)?;
            },
            "moveRequest" => {
                self.parse_move_request()?;
                self.last_move_was_our = true;
            },
            "result" => {
                self.leave();
            },
            _ => {},
        }
        return Ok(())
    }
} 