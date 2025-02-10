use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError, game::{cards::Card, moves::{CarrotsToExchange, JumpCardDetails, GameMove}}, utils::get_attribute::get_attribute};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(super) fn parse_last_move(&mut self, mut parser: EventReader<&[u8]>, attributes: Vec<OwnedAttribute>) -> Result<GameMove, ConnectionHandlerError> {
        match get_attribute(&attributes, "class")?.as_str() {
            "fallback" => return Ok(GameMove::FallBack),
            "eatsalad" => return Ok(GameMove::EatSalad),
            "exchangecarrots" => {
                match get_attribute(&attributes, "amount")?.as_str() {
                    "10" => return Ok(GameMove::ExchangeCarrots(CarrotsToExchange::PlusTen)),
                    "-10" => return Ok(GameMove::ExchangeCarrots(CarrotsToExchange::MinusTen)),
                    c => Err(ConnectionHandlerError::UnexpectedAttributeContent(String::from("10 or -10"), c.to_owned()))
                }
            },
            "advance" => {
                let distance = get_attribute(&attributes, "distance")?.parse::<u8>()?;
                let mut cards: Vec<Card> = Vec::new();
                loop {
                    match parser.next() {
                        Ok(XmlEvent::Characters(s)) => {
                            cards.push(Card::from_str(s.as_str()).expect("This is not a Card!"));
                        }
                        Ok(XmlEvent::EndElement { name }) => {
                            match name.local_name.as_str() {
                                "lastGameMove" => {
                                    break;
                                }
                                _ => {},
                            }
                        }
                        Ok(XmlEvent::EndDocument) => {
                            break
                        },
                        Err(error) => return Err(ConnectionHandlerError::Xml(error)),
                        _ => {}
                    }
                }
                if cards.is_empty() {
                    return Ok(GameMove::Advance(distance));
                }
                return Ok(GameMove::AdvanceWithCards(distance, JumpCardDetails::from_card_list(&cards), cards.pop().unwrap()));
            }
            c => return Err(ConnectionHandlerError::UnexpectedAttributeContent(String::from("fallback, eatsalad, advance or exchangecarrots"), c.to_owned()))
        }
    }
}