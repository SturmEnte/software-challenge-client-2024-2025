use std::fmt::Display;

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

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game State:\n")?;
        write!(f, "  🩷Our Team: {}\n", self.team)?;
        write!(f, "  🔰Starting Team: {}\n", self.start_team)?;
        write!(f, "  🎲Turn: {}\n", self.turn)?;
        write!(f, "  💱Last Carrot Swap: {}\n", self.last_carrot_swap)?;
        
        if let Some(last_move) = &self.last_move {
            write!(f, "  🎞Last Move: {}\n", last_move)?;
        } else {
            write!(f, "  ❌Last Move: None\n")?;
        }
        
        write!(f, "  🟢Your Hare:\n[{}]", self.your_hare)?;
        write!(f, "  🔴Opponent Hare:\n[{}]", self.opponent_hare)?;
        
        Ok(())
    }
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