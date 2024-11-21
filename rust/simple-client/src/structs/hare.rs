use crate::enums::team::Team;
use crate::structs::game_move::Move;

pub struct Hare {
    pub team: Option<Team>,
    pub position: u8,
    pub salads: u8,
    pub carrots: u8, 
    pub last_move: Option<Box<dyn Move>>,
}

impl Hare {
    pub fn new() -> Hare {
        Hare {
            team: None,
            position: 0,
            salads: 0,
            carrots: 0,
            last_move: None,
        }        
    }
}