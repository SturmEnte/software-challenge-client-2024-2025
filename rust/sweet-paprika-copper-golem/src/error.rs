use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionHandlerError {
    #[error("Bord is None")]
    BordIsNone,
    #[error("Game State is None")]
    GameStateIsNone,
    #[error("Room ID is None")]
    RoomIdIsNone,
    #[error("A different content was expected in the attribute: {0}, What was found: {1}")]
    UnexpectedAttributeContent(String, String),
    #[error("An expected element was not found. expected element: {0}")]
    MissingElement(String),
    #[error("An attribute element was not found. expected attribute: {0}")]
    MissingAttribute(String),
    #[error("No Bytes have been written to the Buffer")]
    ZeroBytesReadToBuffer,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Xml(#[from] xml::reader::Error),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    #[error(transparent)]
    GameError(#[from] crate::game::game_error::GameError)
    
}