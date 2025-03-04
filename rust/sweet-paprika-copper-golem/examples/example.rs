use hase_und_igel_client::{computer_player::ComputerPlayer, connection_handler::connection_handler::ConnectionHandler, game::{board::Board, cards::Card, field_type::FieldType, game_state::GameState, moves::{CarrotsToExchange, GameMove, JumpCardDetails}}, utils::triangular_numbers::calculate_triangular_number};

fn main() {

    let mut connection_handler =ConnectionHandler::new(ExampleComputerPlayer::new()).unwrap();
    match connection_handler.join(None) {
        Ok(()) => {},
        Err(e) => eprint!("{}", e),
    }
    match connection_handler.play() {
        Err(error) => eprintln!("{}", error),
        Ok(_) => {}
    }
}

pub struct ExampleComputerPlayer {}

impl ExampleComputerPlayer {
    fn new() -> Self {
        ExampleComputerPlayer {}
    }
}

impl ComputerPlayer for ExampleComputerPlayer {
    fn make_move(&mut self, board: &Board, game_state: &GameState) -> GameMove {
        let mut i: u8 = 0;
        let mut mov = GameMove::Advance(1);

        loop {
            if board.board[game_state.your_hare.position as usize] == FieldType::Salad && !game_state.your_hare.ate_salad_last_round {
                mov = GameMove::EatSalad;
                break;
            }

            i += 1;

            if game_state.your_hare.salads == 0 {
                let distence = 64 - game_state.your_hare.position;
                let carrot_cost = calculate_triangular_number(distence as u16);
                if game_state.your_hare.carrots >= carrot_cost && game_state.your_hare.carrots + 10 <= carrot_cost {
                    return GameMove::Advance(distence)
                }

            }

            if calculate_triangular_number(i as u16) > game_state.your_hare.carrots {
                return GameMove::FallBack;
            }

            if game_state.your_hare.position + i == game_state.opponent_hare.position {continue;}

            match board.board[(i + game_state.your_hare.position) as usize] {
                FieldType::Start => continue,
                FieldType::Carrots => if game_state.your_hare.carrots < 30 {
                    mov = GameMove::Advance(i);
                } else {
                    continue;
                },
                FieldType::Salad => mov = if game_state.your_hare.salads > 0 {
                    GameMove::Advance(i)
                } else {
                    continue;
                },
                FieldType::Position1 => if game_state.your_hare.carrots < 30 {
                    mov = GameMove::Advance(i);
                } else {
                    continue;
                },
                FieldType::Position2 => mov = GameMove::Advance(i),
                FieldType::Hedgehog => continue,
                FieldType::Market => {
                    if calculate_triangular_number(i as u16) + 10 > game_state.your_hare.carrots {continue;}
                    mov = GameMove::AdvanceWithCards(i, JumpCardDetails::new(false, 0), Card::EatSalad);
                },
                FieldType::Hare => if game_state.your_hare.card_eat_salad > 0 && game_state.your_hare.salads > 0 {
                    mov = GameMove::AdvanceWithCards(i, JumpCardDetails::new(false, 0), Card::EatSalad)
                } else {
                    continue;
                },
                FieldType::Goal => {
                    if game_state.your_hare.carrots - calculate_triangular_number(i as u16) <= 10 {return GameMove::Advance(i)}
                    mov = GameMove::Advance(i - 1)
                },
            }
            if game_state.your_hare.position == 63 && game_state.your_hare.carrots > 11 {mov = GameMove::ExchangeCarrots(CarrotsToExchange::MinusTen)}

            if let GameMove::Advance(d) = mov {
                if calculate_triangular_number(d as u16) > game_state.your_hare.carrots {
                    mov = GameMove::FallBack;
                }
            }
            break;
        }
        return mov;
    }
}