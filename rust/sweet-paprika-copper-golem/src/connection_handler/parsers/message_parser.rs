use xml::{reader::XmlEvent, EventReader};

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub fn parse_message(&mut self, mut parser: EventReader<&[u8]>) -> Result<(), ConnectionHandlerError> {
        loop {
            match parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                    match name.local_name.as_str() {
                        "data" => {
                            self.parse_data(parser, &attributes)?;
                            break;
                        },
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(e) => eprintln!("{}", e),
                _ => {},
            }
        }
        Ok(())
    }
}