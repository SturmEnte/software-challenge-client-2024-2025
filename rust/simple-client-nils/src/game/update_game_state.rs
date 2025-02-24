use super::{board::Board, game_error::GameError, game_state::GameState, moves::GameMove};
use handle_moves::*;
use position_reward::give_position_field_reward;

mod handle_moves;
pub(super) mod handle_cards;
mod position_reward;

impl GameState {
    /// A Function that updates the GameState based on a `GameMove`.
    pub fn update(&mut self, board: &Board, mov: GameMove) -> Result<(), GameError> {
        self.turn += 1;
        let current_team = self.get_current_team();
        let (current_hare, opponent_hare) = if current_team == self.team {
            (&mut self.your_hare, &mut self.opponent_hare)
        } else {
            (&mut self.opponent_hare, &mut self.your_hare)
        };
        
        current_hare.ate_salad_last_round = false;
        
        match mov {
            GameMove::FallBack => {
                handle_move_fall_back(current_hare);
            },
            GameMove::EatSalad => {
                handle_move_eat_salad(current_hare, opponent_hare.position)?
            },
            GameMove::ExchangeCarrots(ref carrots_to_exchange) => {
                handle_move_exchange_carrots(current_hare, &carrots_to_exchange);
            },
            GameMove::Advance(distance) => {
                handle_move_advance(current_hare, distance)?;
            },
            GameMove::AdvanceWithCards(distance, ref jumps, ref last_card) => {
                handle_move_advance_with_cards(current_hare, opponent_hare, board, distance, jumps, last_card, &self.turn, &mut self.last_carrot_swap)?;
            },
        }

        give_position_field_reward(board, opponent_hare, current_hare.position);

        self.last_move = Some(mov);
        println!("{:?}", self);
        return Ok(());
    }
}



