use crate::{computer_player::ComputerPlayer, error::ConnectionHandlerError, game::{board::Board, field_type::FieldType, game_state::GameState}};

use super::connection_handler::ConnectionHandler;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(crate) fn get_bord(&self) -> Result<&Board, ConnectionHandlerError> {
        return self.bord.as_ref().ok_or(ConnectionHandlerError::BordIsNone);
    }

    pub(crate) fn get_game_state(&self) -> Result<&GameState, ConnectionHandlerError> {
        return self.game_state.as_ref().ok_or(ConnectionHandlerError::GameStateIsNone);
    }

    pub(crate) fn get_game_state_mut(&mut self) -> Result<&mut GameState, ConnectionHandlerError> {
        return self.game_state.as_mut().ok_or(ConnectionHandlerError::GameStateIsNone);
    }
}
