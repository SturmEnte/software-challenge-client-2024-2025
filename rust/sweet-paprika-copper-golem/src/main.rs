use hase_und_igel_client::{game, prelude::*};

fn main() {
    let player: SweetPaprikaCopperGolem = SweetPaprikaCopperGolem {};
    let mut con_handler = ConnectionHandler::new(player).unwrap();
    con_handler.join(None).unwrap();
    con_handler.play().unwrap();
}

struct SweetPaprikaCopperGolem {
    
}

impl ComputerPlayer for SweetPaprikaCopperGolem {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        let mut moves: Vec<GameMove> = calculate_legal_moves(game_state, board);

        moves.pop().unwrap()
    }
}