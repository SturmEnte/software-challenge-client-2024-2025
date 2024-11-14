use super::board::Board;
use super::hare::Hare;
use crate::enums::team::Team;

pub struct GameData {
    pub initialized: bool,
    pub board: Board,
    pub our_hare: Hare,
    pub enemy_hare: Hare,
    pub start_team: Team,
    pub turn: i8,
    pub game_over: bool,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { initialized: false, board: Board::new(), start_team: Team::Undefined, our_hare: Hare::new(), enemy_hare: Hare::new(), turn: 0, game_over: false }
    }

    pub fn set_team(&mut self, team: &str) {
        if team == "ONE" {
            self.our_hare.team = Team::One;
            self.enemy_hare.team = Team::Two;
        } else {
            self.enemy_hare.team = Team::One;
            self.our_hare.team = Team::Two;
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