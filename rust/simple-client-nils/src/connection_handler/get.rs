use crate::{computer_player::ComputerPlayer, error::ConnectionHandlerError, game::game_state::GameState};

use super::connection_handler::ConnectionHandler;

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(crate) fn get_game_state_mut(&mut self) -> Result<&mut GameState, ConnectionHandlerError> {
        return self.game_state.as_mut().ok_or(ConnectionHandlerError::GameStateIsNone);
    }
}
