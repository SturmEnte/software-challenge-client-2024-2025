use hase_und_igel_client::prelude::*;

fn main() {
    let mut connection_handler = ConnectionHandler::from_commandline_args_and_join(SweetPaprikaOstrich{}).unwrap();
    connection_handler.play().unwrap();
}

struct SweetPaprikaOstrich {}

impl ComputerPlayer for SweetPaprikaOstrich {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        let mut legal_moves = calculate_legal_moves(game_state, board);
        let length = legal_moves.len();
        let mut evaluations = vec![std::i32::MIN; length];
        let mut game_states = vec![game_state.clone(); length];
        for i in 0..length {
            game_states[i].update(board, legal_moves[i].clone()).unwrap();
        }
        for (i, new_game_state) in game_states.iter().enumerate() {
            evaluations[i] = minimax(new_game_state, board, 5, false)
        }
        return legal_moves.swap_remove(evaluations.into_iter().enumerate().max_by_key(|(_, value)| *value).map(|(i, _)| i).unwrap());
    }
}

fn minimax(game_state: &GameState, board: &Board, depth: usize, maximizing_player: bool) -> i32 {
    if depth == 0 || (game_state.turn == 60) || (game_state.your_hare.position == 64 ||game_state.opponent_hare.position == 64) {return eval(game_state);}

    if maximizing_player {
        let legal_moves = calculate_legal_moves(game_state, board);
        let length = legal_moves.len();
        let mut evaluations = vec![std::i32::MIN; length];
        let mut game_states = vec![game_state.clone(); length];
        for i in 0..length {
            game_states[i].update(board, legal_moves[i].clone()).unwrap();
        }
        for (i, new_game_state) in game_states.iter().enumerate() {
            evaluations[i] = minimax(new_game_state, board, depth - 1, false)
        }
        if legal_moves.is_empty() {
            let mut new_game_state = game_state.clone();
            new_game_state.turn += 1;
            evaluations.push(minimax(&new_game_state, board, depth, true));
        }
        return *evaluations.iter().max().unwrap();
    } else {
        let legal_moves = calculate_legal_moves(game_state, board);
        let length = legal_moves.len();
        let mut evaluations = vec![std::i32::MAX; length];
        let mut game_states = vec![game_state.clone(); length];
        for i in 0..length {
            game_states[i].update(board, legal_moves[i].clone()).unwrap();
        }
        for (i, new_game_state) in game_states.iter().enumerate() {
            evaluations[i] = minimax(new_game_state, board, depth - 1, true)
        }
        if legal_moves.is_empty() {
            let mut new_game_state = game_state.clone();
            new_game_state.turn += 1;
            evaluations.push(minimax(&new_game_state, board, depth, true));
        }
        return *evaluations.iter().min().unwrap();
    }
}

fn eval(game_state: &GameState) -> i32 {
    let mut eval = 0;
    if game_state.your_hare.position == 64 {
        eval = std::i32::MAX;
        eval -= game_state.turn as i32 * 11;
        eval -= game_state.your_hare.carrots as i32;
    }
    if game_state.opponent_hare.position == 64 {eval -= 2000000}
    eval += game_state.opponent_hare.salads as i32 * 5;
    eval -= game_state.your_hare.salads as i32 * 130;
    if game_state.your_hare.salads != 0 {
        eval += match game_state.your_hare.card_eat_salad{
        0 => 0,
        1 => 90,
        2 => 130,
        _ => 110
        }
    } else {
        eval += game_state.your_hare.position as i32 * 5
    }
    eval += match game_state.your_hare.position {
        0..20 => game_state.your_hare.position as i32,
        20..40 => game_state.your_hare.position as i32 * 5,
        40..64 => game_state.your_hare.position as i32 * 10,
        _ => 0,
    };
    eval
}