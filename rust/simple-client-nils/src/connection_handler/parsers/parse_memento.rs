use xml::{reader::XmlEvent, EventReader};

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError, game::{board::Board, game_state::GameState, team::Team}, utils::get_attribute::get_attribute};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(super) fn parse_memento(&mut self, mut parser: EventReader<&[u8]>) -> Result<(), ConnectionHandlerError> {
            loop {
                match parser.next() {
                    Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                        match name.local_name.as_str() {
                            "state" => {
                                let game_state: &mut GameState = self.get_game_state_mut()?;
                                if game_state.turn != 0 {continue}
                                if get_attribute(&attributes, "startTeam")?.as_str() == "TWO" {
                                    game_state.start_team = Team::Two;
                                }
                            }
                            "board" => {
                                if self.bord.is_some() {
                                    parser.skip()?;
                                    continue;
                                }
                                self.bord = Some(Board::new(self.parse_board(&mut parser)?));
                                #[cfg(feature = "debug_board_info")]
                                println!("{}", self.bord.as_ref().unwrap());
                            },
                            "lastMove" => {
                                if self.last_move_was_our {
                                    self.last_move_was_our = false;
                                    break;
                                }
                                let mov = self.parse_last_move(parser, attributes)?;
                                self.update_game_state(mov)?;
                                break;
                            },
                            _ => {},
                        }
                    }
                    Ok(XmlEvent::EndDocument) => break,
                    Err(error) => return Err(ConnectionHandlerError::Xml(error)),
                    _ => {}
                }
            }
        Ok(())
    }
}