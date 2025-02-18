
use super::{hare::Hare, moves::GameMove, team::Team};

#[derive(Debug)]
pub struct GameState {
    pub team: Team,
    pub start_team: Team,
    pub turn: u8,
    pub last_carrot_swap: u8,
    pub last_move: Option<GameMove>,
    pub your_hare: Hare,
    pub opponent_hare: Hare,
}

impl GameState {
    pub fn new(team: Team) -> Self {
        GameState {
            your_hare: Hare::new(),
            opponent_hare: Hare::new(),
            last_carrot_swap: 0,
            turn: 1,
            team: team,
            start_team: Team::One,
            last_move: None,
        }
    }

    pub fn get_hare_by_team(&self, team: &Team) -> &Hare {
        return if self.team == *team {
            &self.your_hare
        } else {
            &self.opponent_hare
        }
    }

    pub fn get_hare_by_team_mut(&mut self, team: &Team) -> &mut Hare {
        return if self.team == *team {
            &mut self.your_hare
        } else {
            &mut self.opponent_hare
        }
    }

    pub fn get_current_team(&self) -> Team {
        if self.turn % 2 == 0 {
            self.start_team
        } else {
            !self.start_team
        }
    }

    pub fn get_next_team(&self) -> Team {
        if (self.turn + 1) % 2 == 0 {
            self.start_team
        } else {
            !self.start_team
        }
    }
}