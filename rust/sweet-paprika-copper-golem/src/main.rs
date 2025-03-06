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
        println!("Move");
        let moves: Vec<GameMove> = calculate_legal_moves(game_state, board);

        let mut best_move: Option<GameMove> = None;
        let mut best_moves_eval: i32 = std::i32::MIN;

        for mov in moves {
            // let mut new_game_state = game_state.clone();
            // new_game_state.update(board, mov.clone()).unwrap();

            let eval: i32 = minimax(&mov, game_state.clone(), board, 3, false); // I need to check later if false is correct
            
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

    let mut new_game_state: GameState = game_state.clone();
    new_game_state.update(board, mov.clone()).unwrap();

    // Check if the game ended
    // I also need to check for a player in the goal and if the other player cant also go on it
    if depth == 0 {
        return evaluate(&game_state, board, &mov); 
    }

    if maximizing_player {
        let mut max_eval: i32 = std::i32::MIN;
        let moves = calculate_legal_moves(&new_game_state, board);
        for new_mov in moves {
    
            let eval: i32 = minimax(&new_mov, new_game_state.clone(), board, depth - 1, false);
            max_eval = std::cmp::max(max_eval, eval);
        }
        return max_eval;
    } else {
        let mut min_eval: i32 = std::i32::MAX;
        let moves = calculate_legal_moves(&new_game_state, board);
        for new_mov in moves {
            let eval: i32 = minimax(&new_mov, new_game_state.clone(), board, depth - 1, true);
            min_eval = std::cmp::min(min_eval, eval);
        }
        return min_eval;
    }
}

// Constants for evaluation
const SALAD_MULTIPLIER: i32 = 10;

// Eveluate a game state
fn evaluate(game_state: &GameState, board: &Board, m: &GameMove) -> i32 {
    let mut eval: i32 = 0;

    // Check if we are on the goal
    if board.board[game_state.your_hare.position as usize] == FieldType::Goal {
        return std::i32::MAX;
    }

    // Subtrackt from the score for the remainning salads
    eval += -(game_state.your_hare.salads as i32) * SALAD_MULTIPLIER;

    // Add for position
    eval += game_state.your_hare.position as i32;

    // Carrots
    if game_state.your_hare.position > 63 {
        eval -= game_state.your_hare.carrots as i32 - 10;
    } else {
        eval -= game_state.your_hare.carrots as i32 - 40;
    }

    eval
}