use crate::structs::board::Board;
use crate::enums::team::Team;

pub struct GameData {
    pub initialized: bool,
    pub board: Board,
    pub start_team: Team,
    pub our_team: Team,
    pub opponent_team: Team,
    pub turn: i8,
    pub game_over: bool,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { initialized: false, board: Board::new(), start_team: Team::Undefined, our_team: Team::Undefined, opponent_team: Team::Undefined, turn: 0, game_over: false }
    }

    pub fn set_team(&mut self, team: &str) {
        if team == "ONE" {
            self.our_team = Team::One;
            self.opponent_team = Team::Two;
        } else {
            self.opponent_team = Team::One;
            self.our_team = Team::Two;
        }
    }

    pub fn set_start_team(&mut self, team: &str) {
        if team == "ONE" {
            self.start_team = Team::One;
        } else {
            self.start_team = Team::Two;
        }
    }
}