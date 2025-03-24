use std::time::{SystemTime, UNIX_EPOCH};

use hase_und_igel_client::prelude::*;

fn main() {
    let player: SweetPaprikaVulture = SweetPaprikaVulture {};
    let mut con_handler = ConnectionHandler::from_commandline_args_and_join(player).unwrap();
    con_handler.play().unwrap();
}

struct SweetPaprikaVulture {}

const MAX_DEPTH: u8 = 50;
const COMPUTION_MILLIS: u128 = 1900;

impl ComputerPlayer for SweetPaprikaVulture {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        println!("---Move Request---");
        
        let timestamp: u128 = current_timestamp_millis();
        
        let moves: Vec<GameMove> = calculate_legal_moves(game_state, board);

        let mut best_move: Option<GameMove> = None;
        let mut best_moves_eval: i32 = std::i32::MIN;

        // The depth starts at 2 because that should always be possible to calculate
        let mut depth: u8 = 2;

        while (current_timestamp_millis() - timestamp) < COMPUTION_MILLIS {

            if depth > MAX_DEPTH {
                println!("Max depth reached!");
                break;
            }

            let mut local_best_move: Option<GameMove> = None;
            let mut local_best_moves_eval: i32 = std::i32::MIN;

            for mov in moves.clone() {

                let mut new_game_state = game_state.clone();
                new_game_state.update(board, mov.clone()).unwrap();

                let eval: i32 = minimax(new_game_state, board, depth, false, std::i32::MIN, std::i32::MAX, &timestamp); // I need to check later if false is correct
                
                if eval > local_best_moves_eval {
                    local_best_moves_eval = eval;
                    local_best_move = Some(mov)
                }

                // This is to preven an unfinished layer to be processed
                if (current_timestamp_millis() - timestamp) >= COMPUTION_MILLIS {
                    break;
                }
            }
            
            // This is to preven an unfinished layer to be processed
            if (current_timestamp_millis() - timestamp) >= COMPUTION_MILLIS {
                break;
            }

            if local_best_moves_eval > best_moves_eval {
                best_moves_eval = local_best_moves_eval;
                best_move = local_best_move;
            }

            println!("Finished calculating depth: {}", depth);

            depth += 1;
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

fn minimax(mut game_state: GameState, board: &Board, depth: u8, maximizing_player: bool, alpha: i32, beta: i32, start_timestamp: &u128) -> i32 {

    // If the max depth is reached, the time is up or both hares are on the goal, then the game state is evaluated
    if depth == 0 || start_timestamp + COMPUTION_MILLIS <= current_timestamp_millis() || (game_state.your_hare.position == 64 || game_state.opponent_hare.position == 64) {
        return evaluate(&game_state); 
    }

    let legal_moves: Vec<GameMove> = calculate_legal_moves(&game_state, board);

    // Handle skipping rounds
    if legal_moves.is_empty() {
        game_state.turn += 1;
        return minimax(game_state, board, depth - 1, !maximizing_player, alpha, beta, start_timestamp);
    }

    if maximizing_player {
        let mut max_eval: i32 = std::i32::MIN;
        let mut new_alpha: i32 = alpha;

        for new_mov in legal_moves {

            let mut new_game_state = game_state.clone();
            new_game_state.update(board, new_mov).unwrap();

            let eval: i32 = minimax(new_game_state, board, depth - 1, false, new_alpha, beta, start_timestamp);
            max_eval = std::cmp::max(max_eval, eval);

            new_alpha = std::cmp::max(new_alpha, max_eval);
            if beta <= new_alpha {
                break;
            }
        }

        return max_eval;
    } else {
        let mut min_eval: i32 = std::i32::MAX;
        let mut new_beta = beta;

        for new_mov in legal_moves {

            let mut new_game_state = game_state.clone();
            new_game_state.update(board, new_mov).unwrap();

            let eval: i32 = minimax(new_game_state, board, depth - 1, true, alpha, new_beta, start_timestamp);
            min_eval = std::cmp::min(min_eval, eval);

            new_beta = std::cmp::min(new_beta, min_eval);
            if new_beta <= alpha {
                break;
            }
        }

        return min_eval;
    }
}

// Eveluate a game state
fn evaluate(game_state: &GameState) -> i32 {
    let mut eval: i32 = 0;

    if game_state.your_hare.position == 64 {
        eval = std::i32::MAX;
        eval -= game_state.turn as i32 * 11;
        eval -= game_state.your_hare.carrots as i32;
    }

    if game_state.opponent_hare.position == 64 {
        eval -= 2000000
    }
    
    eval += game_state.opponent_hare.salads as i32 * 5;
    eval -= game_state.your_hare.salads as i32 * 130;
    
    if game_state.your_hare.salads != 0 { 
        eval += match game_state.your_hare.card_eat_salad{
            0 => 0,
            1 => 100,
            2 => 150,
            _ => 130
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

    eval += game_state.turn as i32 * 10;
    
    eval
}

fn current_timestamp_millis() -> u128 {
    let now: SystemTime = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_millis()
}