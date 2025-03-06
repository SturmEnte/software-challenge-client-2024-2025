use hase_und_igel_client::prelude::*;

fn main() {
    let mut connection_handler = ConnectionHandler::from_commandline_args_and_join(SweetPaprikaOstrich{}).unwrap();
    connection_handler.play().unwrap();
}

struct SweetPaprikaOstrich {}

impl ComputerPlayer for SweetPaprikaOstrich {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        todo!()
    }
}