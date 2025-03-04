use xml::{reader::XmlEvent, EventReader};

use crate::utils::get_attribute::get_attribute;
use crate::error::ConnectionHandlerError;

pub(crate) fn parse_joined(parser: EventReader<&[u8]>) -> Result<Box<str>, ConnectionHandlerError> {
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if name.local_name == "joined" {return Ok(get_attribute(&attributes, "roomId")?.into_boxed_str())};
            },
            Err(error) => return Err(ConnectionHandlerError::Xml(error)),
            _ => {}
        }
    }
    return Err(ConnectionHandlerError::MissingElement(String::from("joined")));
}