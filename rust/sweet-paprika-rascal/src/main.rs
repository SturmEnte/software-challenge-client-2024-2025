use hase_und_igel_client::prelude::*;
fn main() {
    let mut connection_handler = ConnectionHandler::new(TestPlayer {}).unwrap();
    let _ = connection_handler.join(None);
    let _ = connection_handler.play();
}

struct SweetPaprikaRascal{}

impl ComputerPlayer for SweetPaprikaRascal {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        let mut legal_moves = calculate_legal_moves(game_state, board);
        let mut game_states = vec![game_state.clone(); legal_moves.len()];
        let mut evaluations = vec![0; legal_moves.len()];
        for i in 0..legal_moves.len() {
            game_states[i].update(board, legal_moves[i].clone()).unwrap();
        }
        for (new_game_state, evaluation) in game_states.iter().zip(evaluations.iter_mut())  {
            *evaluation += new_game_state.your_hare.salads as i32 * -400 + 2000;
            *evaluation += new_game_state.your_hare.position as i32 - game_state.your_hare.position as i32 * 10;
            if new_game_state.your_hare.position >= 64 {*evaluation += 10000;}
            if new_game_state.your_hare.position >= 60 {*evaluation += 100 - new_game_state.your_hare.carrots as i32;}
            if new_game_state.your_hare.salads != 0 && new_game_state.your_hare.card_eat_salad > game_state.your_hare.card_eat_salad {*evaluation += 30000000;}
        }
        return legal_moves.swap_remove(evaluations.into_iter().enumerate().max_by_key(|(_, value)| *value).map(|(i, _)| i).unwrap());
    }
}