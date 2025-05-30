use xml::{attribute::OwnedAttribute, EventReader};

use crate::connection_handler::connection_handler::GameMessage;
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
                if self.last_game_message == GameMessage::OurLastMove || self.last_game_message == GameMessage::OurLastMoveOpponentTurnSkipped {
                    self.game_state.as_mut().unwrap().turn += 1;
                }
                self.parse_move_request()?;
                self.last_game_message.move_request_receive();
            },
            "result" => {
                self.leave();
            },
            _ => {},
        }
        return Ok(())
    }
} 