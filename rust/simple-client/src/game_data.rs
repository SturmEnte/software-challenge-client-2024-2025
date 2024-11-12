use crate::board::Board;

pub struct GameData {
    pub initialized: bool,
    pub board: Board,
    pub start_team: i8,
    pub our_team: i8,
    pub opponent_team: i8,
    pub turn: i8,
    pub game_over: bool,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { initialized: false, board: None, start_team: 0, our_team: 0, opponent_team: 0, turn: 1, game_over: false }
    }

    pub fn set_team(&mut self, team: &str) {
        if team == "ONE" {
            self.our_team = 1;
            self.opponent_team = 2;
        } else {
            self.opponent_team = 1;
            self.our_team = 2;
        }
    }

    pub fn set_start_team(&mut self, team: &str) {
        if team == "ONE" {
            self.start_team = 1;
        } else {
            self.start_team = 2;
        }
    }
}