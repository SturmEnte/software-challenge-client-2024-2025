use hase_und_igel_client::prelude::*;

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
        let moves: Vec<GameMove> = calculate_legal_moves(game_state, board);

        let mut best_move: Option<GameMove> = None;
        let mut best_moves_eval: i32 = std::i32::MIN;

        for mov in moves {
            let eval: i32 = minimax(&mov, game_state.clone(), board, 2, false); // I need to check later if false is correct
            
            println!("{}", eval);
            
            if eval > best_moves_eval {
                best_moves_eval = eval;
                best_move = Some(mov)
            }
        }

        // Return the best move if there was one
        if best_move.is_some() {
            return best_move.unwrap();
        }

        // Return the first move if there was no evaluated move
        println!("No move found after minimax");
        calculate_legal_moves(game_state, board)[0].clone()
    }
}

fn minimax(mov: &GameMove, game_state: GameState, board: &Board, depth: u8, maximizing_player: bool) -> i32 {
    // Check if the game ended
    // I also need to check for a player in the goal and if the other player cant also go on it
    if depth == 0 {
        return evaluate(&game_state, &mov); 
    }

    if maximizing_player {
        let mut max_eval: i32 = std::i32::MIN;
        let moves = calculate_legal_moves(&game_state, board);
        for new_mov in moves {
            let eval: i32 = minimax(&new_mov, game_state.clone(), board, depth - 1, false);
            max_eval = std::cmp::max(max_eval, eval);
        }
        return max_eval;
    } else {
        let mut min_eval: i32 = std::i32::MAX;
        let moves = calculate_legal_moves(&game_state, board);
        for new_mov in moves {
            let eval: i32 = minimax(&new_mov, game_state.clone(), board, depth - 1, true);
            min_eval = std::cmp::min(min_eval, eval);
        }
        return min_eval;
    }
}

// Eveluate a game state
fn evaluate(game_state: &GameState, m: &GameMove) -> i32 {
    0
}