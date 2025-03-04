#[cfg(feature = "log_incoming_xml")]
use std::{fs::{OpenOptions, File}, env};

use std::net::{TcpStream, ToSocketAddrs};
use crate::{computer_player::ComputerPlayer, error::ConnectionHandlerError, game::{board::Board, game_state::GameState, moves::GameMove}};

/// A struct which manages the communication with the game server.
/// /// # Example
///
/// ```
/// // Assuming `MyComputerPlayer` implements the `ComputerPlayer` trait
/// let player = MyComputerPlayer::new();
/// let mut connectionhandler = ConnectionHandler::new(player).unwrap();
///
/// 
/// connectionhandler.join(Some("reservation_code")).unwrap();
/// connectionhandler.play().unwrap();
/// ```
pub struct ConnectionHandler<C: ComputerPlayer> {
    pub(super) connected: bool,
    pub(super) connection: TcpStream,
    pub(super) room_id: Option<Box<str>>,
    pub(super) bord:Option<Board>,
    pub(super) game_state: Option<GameState>,
    pub(super) player: C,
    pub(super) last_game_message: GameMessage,
    #[cfg(feature = "log_incoming_xml")]
    pub(super) xml_input_file: Option<File> // This field is locked behind the feature flag log_incoming_xml.
}

#[derive(PartialEq)]
pub(super) enum GameMessage {
    StartMessage,
    OurLastMove,
    OurLastMoveOpponentTurnSkipped,
    OpponentLastMove,
    OpponentLastMoveOurTurnSkipped,
    MoveRequest,
    MoveRequestOpponentTurnSkipped,
}

impl <C: ComputerPlayer> ConnectionHandler<C> {
        /// Creates a new `ConnectionHandler` connected to the default address.
        ///
        /// # Arguments
        ///
        /// * `player` - A struct which implements `ComputerPlayer`.
        ///
        /// # Returns
        ///
        /// * `Result<Self, ConnectionHandlerError>` - A result containing the new `ConnectionHandler` 
    pub fn new(player: C) -> Result<Self, ConnectionHandlerError>{
        Ok(
            ConnectionHandler{
                connected: false,
                connection: TcpStream::connect("127.0.0.1:13050")?,
                room_id: None,
                player: player,
                bord: None,
                game_state: None,
                last_game_message: GameMessage::StartMessage,
                #[cfg(feature = "log_incoming_xml")]
                xml_input_file: OpenOptions::new()
                    .write(true)
                    .read(false)
                    .create(true)
                    .truncate(true)
                    .open(env::var("XML_LOG_DIR").unwrap_or("".to_string()) + "incoming_xml_log.txt").ok()
            }
        )
    }
    /// Creates a new `ConnectionHandler` connectied to a given address.
    ///
    /// # Arguments
    ///
    /// * `player` - A struct which implements `ComputerPlayer`.
    /// * `addr` - The address to connect to, which implements `ToSocketAddrs`.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ConnectionHandlerError>` - A result containing the new `ConnectionHandler` 
    pub fn from_addres(player: C, addr: &impl ToSocketAddrs) -> Result<Self, ConnectionHandlerError>{
        Ok(
            ConnectionHandler{
                connected: false,
                connection: TcpStream::connect(addr)?,
                room_id: None,
                player: player,
                bord: None,
                game_state: None,
                last_game_message: GameMessage::StartMessage,
                #[cfg(feature = "log_incoming_xml")]
                xml_input_file: OpenOptions::new()
                    .write(true)
                    .read(false)
                    .create(true)
                    .truncate(true)
                    .open(env::var("XML_LOG_DIR").unwrap_or("".to_string()) + "incoming_xml_log.txt").ok()
            }
        )
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn leave(&mut self) {
        self.connected = false;
    }

    pub(crate) fn player_move(&mut self) -> Result<GameMove, ConnectionHandlerError> {
        return Ok(self.player.make_move(self.bord.as_ref().ok_or(ConnectionHandlerError::BordIsNone)?, self.game_state.as_ref().ok_or(ConnectionHandlerError::GameStateIsNone)?))
    }

    pub(crate) fn update_game_state(&mut self, mov: GameMove) -> Result<(), ConnectionHandlerError> {
        self.game_state.as_mut().ok_or(ConnectionHandlerError::GameStateIsNone)?.update(self.bord.as_ref().ok_or(ConnectionHandlerError::BordIsNone)?, mov)?;
        #[cfg(feature = "debug_game_state_info")]
        println!("{}", self.game_state.as_ref().unwrap());
        Ok(())
    }
}

impl GameMessage {
    pub(super) fn last_move_receive(&mut self) {
        *self = match self {
            GameMessage::StartMessage => Self::OpponentLastMove,
            GameMessage::OurLastMove => Self::OpponentLastMove,
            GameMessage::OurLastMoveOpponentTurnSkipped => Self::OpponentLastMove,
            GameMessage::OpponentLastMove => Self::OpponentLastMoveOurTurnSkipped,
            GameMessage::OpponentLastMoveOurTurnSkipped => Self::OpponentLastMove,
            GameMessage::MoveRequest => Self::OurLastMove,
            GameMessage::MoveRequestOpponentTurnSkipped => Self::OurLastMoveOpponentTurnSkipped,
        }
    }

    pub(super) fn move_request_receive(&mut self) {
        *self = match self {
            GameMessage::StartMessage => Self::MoveRequest,
            GameMessage::OurLastMove => Self::MoveRequestOpponentTurnSkipped,
            GameMessage::OurLastMoveOpponentTurnSkipped => Self::MoveRequest,
            GameMessage::OpponentLastMove => Self::MoveRequest,
            GameMessage::OpponentLastMoveOurTurnSkipped => Self::MoveRequest,
            GameMessage::MoveRequest => Self::MoveRequestOpponentTurnSkipped,
            GameMessage::MoveRequestOpponentTurnSkipped => Self::MoveRequest,
        }
    }
}