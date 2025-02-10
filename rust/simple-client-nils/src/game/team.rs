use std::ops::Not;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Team {
    One,
    Two,
}

impl Not for Team {
    type Output = Team;
    fn not(self) -> Team {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }
}