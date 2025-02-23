use super::board::Board;
use super::hare::Hare;
use crate::enums::team::Team;

#[derive(Clone)]
pub struct GameData {
    pub room_id: String,
    pub board: Board,
    pub our_hare: Hare,
    pub enemy_hare: Hare,
    pub start_team: Option<Team>,
    pub turn: u8,
    pub game_over: bool,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { room_id: String::new(), board: Board::new(), start_team: None, our_hare: Hare::new(), enemy_hare: Hare::new(), turn: 0, game_over: false }
    }

    pub fn set_team(&mut self, team: &str) {
        if team == "ONE" {
            self.our_hare.team = Some(Team::One);
            self.enemy_hare.team = Some(Team::Two);
        } else {
            self.enemy_hare.team = Some(Team::One);
            self.our_hare.team = Some(Team::Two);
        }
    }

    pub fn set_start_team(&mut self, team: &str) {
        if team == "ONE" {
            self.start_team = Some(Team::One);
        } else {
            self.start_team = Some(Team::Two);
        }
    }
}