use std::io::Write;

use crate::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, error::ConnectionHandlerError, game::moves::GameMove};

impl<C: ComputerPlayer> ConnectionHandler<C> {
    pub(crate) fn parse_move_request(&mut self) -> Result<(), ConnectionHandlerError> {

        const MESSAGE_START: &str = "<room roomId=\"";
        const MESSAGE_END: &str = "\n</room>";
        
        let mov = self.player_move()?;

        self.connection.write(match &mov {
            GameMove::FallBack => format!("{}{}\">\n  <data class=\"fallback\"/>{}", MESSAGE_START, self.room_id.as_ref().ok_or(ConnectionHandlerError::RoomIdIsNone)?, MESSAGE_END),
            GameMove::EatSalad => format!("{}{}\">\n  <data class=\"eatsalad\"/>{}", MESSAGE_START, self.room_id.as_ref().ok_or(ConnectionHandlerError::RoomIdIsNone)?, MESSAGE_END),
            GameMove::Advance(distance) => format!("{}{}\">\n  <data class=\"advance\" distance=\"{}\"/>{}", MESSAGE_START, self.room_id.as_ref().ok_or(ConnectionHandlerError::RoomIdIsNone)?, distance, MESSAGE_END),
            GameMove::ExchangeCarrots(carrots) => {
                let amount = match carrots {
                    crate::game::moves::CarrotsToExchange::MinusTen => "-10",
                    crate::game::moves::CarrotsToExchange::PlusTen => "10",
                };
                format!("{}{}\">\n  <data class=\"exchangecarrots\" amount=\"{}\"/>{}", MESSAGE_START, self.room_id.as_ref().ok_or(ConnectionHandlerError::RoomIdIsNone)?, amount, MESSAGE_END)
            },
            GameMove::AdvanceWithCards(distance, jumps, last_card) => {
                let mut message = String::from(format!("{}{}\">\n  <data class=\"advance\" distance=\"{}\">", MESSAGE_START, self.room_id.as_ref().ok_or(ConnectionHandlerError::RoomIdIsNone)?, distance));
                if jumps.get_number_of_jumps() == 0 {
                    message += &format!("\n    <card>{}</card>", last_card.to_string());
                } else {
                    let mut number_of_jumps = jumps.get_number_of_jumps();
                    //Specifies whether a hurry ahead card should be added or a fall back card.
                    // true = hurry ahead
                    // false = fall back
                    let mut card_to_add = jumps.is_first_card_hurry_ahead();
                    while number_of_jumps > 0 {
                        if card_to_add {
                            message += "\n    <card>HURRY_AHEAD</card>";
                            number_of_jumps -= 1;
                            card_to_add = !card_to_add;
                        } else {
                            message += "\n    <card>FALL_BACK</card>";
                        }
                    }
                }
                message += "\n  </data>";
                message += MESSAGE_END;
                println!("{}", message);
                message
            },
        }.as_bytes())?;

        self.update_game_state(&mov)?;

        return Ok(());
    }
}