use std::{ops::Not, fmt::Display};

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

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::One => write!(f, "ONE"),
            Team::Two => write!(f, "TWO"),
        }
    }
}