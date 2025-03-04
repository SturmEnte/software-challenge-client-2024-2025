use std::mem::MaybeUninit;

use xml::EventReader;

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError, game::field_type::FieldType, utils::string_to_field_type::string_to_field_type};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    #![allow(invalid_value)]
    pub(super) fn parse_board(&mut self, parser: &mut EventReader<&[u8]>) -> Result<[FieldType; 65], ConnectionHandlerError> {
        let mut board: [FieldType; 65] = unsafe {MaybeUninit::uninit().assume_init()};
        let mut board_index: u8 = 0;
        loop {
            match parser.next() {
                Ok(xml::reader::XmlEvent::EndElement { name }) => if name.local_name == "board" {break},
                Ok(xml::reader::XmlEvent::Characters(text)) => {
                    match string_to_field_type(text) {
                        Some(field) => {
                            board[board_index as usize] = field;
                            board_index += 1;
                        },
                        None => {},
                    }
                },
                _ => {},
            }
        }
        if board_index < 65 {return Err(ConnectionHandlerError::MissingElement(String::from("field")));}
        return Ok(board);
    }
}