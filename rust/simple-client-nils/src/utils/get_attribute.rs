use xml::attribute::OwnedAttribute;

use crate::error::ConnectionHandlerError;

pub(crate) fn get_attribute(attributes: &Vec<OwnedAttribute>, wanted_attribute: &str) -> Result<String, ConnectionHandlerError> {
    for a in attributes {
        if a.name.to_string() == wanted_attribute {
            return Ok(a.value.to_string());
        }
    }
    return Err(ConnectionHandlerError::MissingAttribute(String::from(wanted_attribute)));
}