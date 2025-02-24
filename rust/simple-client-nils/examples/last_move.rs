use hase_und_igel_client::prelude::*;

fn main() {
    let mut connection_handler = ConnectionHandler::new(LastMovePlayer{}).unwrap();
    let _ = connection_handler.join(None);
    let _ = connection_handler.play();
}

struct LastMovePlayer{}

impl ComputerPlayer for LastMovePlayer {
    fn make_move(&mut self, board: &Board, game_state: &hase_und_igel_client::game::game_state::GameState) -> hase_und_igel_client::game::moves::GameMove {
        println!("{:?}", calculate_legal_moves(game_state, board));
        return calculate_legal_moves(game_state, board).pop().unwrap();
    }
}