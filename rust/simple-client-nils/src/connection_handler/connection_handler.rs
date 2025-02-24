#[cfg(feature = "log_incoming_xml")]
use std::{fs::{OpenOptions, File}, env};

use std::net::{TcpStream, ToSocketAddrs};
use crate::{computer_player::ComputerPlayer, error::ConnectionHandlerError, game::{board::Board, game_state::GameState, moves::GameMove}};

pub struct ConnectionHandler<C: ComputerPlayer> {
    pub(super) connected: bool,
    pub(super) connection: TcpStream,
    pub(super) room_id: Option<Box<str>>,
    pub(super) bord:Option<Board>,
    pub(super) game_state: Option<GameState>,
    pub(super) player: C,
    pub(super) last_move_was_our: bool,
    #[cfg(feature = "log_incoming_xml")]
    pub(super) xml_input_file: Option<File> // This field is locked behind the feature flag log_incoming_xml.
}

impl <C: ComputerPlayer> ConnectionHandler<C> {
    pub fn new(player: C) -> Result<Self, ConnectionHandlerError>{
        Ok(
            ConnectionHandler{
                connected: false,
                connection: TcpStream::connect("127.0.0.1:13050")?,
                room_id: None,
                player: player,
                bord: None,
                game_state: None,
                last_move_was_our: false,
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
    pub fn from_addres(player: C, addr: &impl ToSocketAddrs) -> Result<Self, ConnectionHandlerError>{
        Ok(
            ConnectionHandler{
                connected: false,
                connection: TcpStream::connect(addr)?,
                room_id: None,
                player: player,
                bord: None,
                game_state: None,
                last_move_was_our: false,
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
        Ok(())
    }
}