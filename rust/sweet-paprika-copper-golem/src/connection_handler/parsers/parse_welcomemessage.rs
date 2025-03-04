use xml::attribute::OwnedAttribute;

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError, game::{game_state::GameState, team::Team}, utils::get_attribute::get_attribute};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(super) fn parse_welcomemessage(&mut self, attributes: &Vec<OwnedAttribute>) -> Result<(), ConnectionHandlerError> {
        match get_attribute(attributes, "color")?.as_str() {
            "ONE" => self.game_state = Some(GameState::new(Team::One)),
            "TWO" => self.game_state = Some(GameState::new(Team::Two)),
            c => return Err(ConnectionHandlerError::UnexpectedAttributeContent(String::from("color"), String::from(c))),
        }
        Ok(())
    }
}