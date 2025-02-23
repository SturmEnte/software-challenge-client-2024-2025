use std::vec::Vec;

use crate::enums::card::Card;
use crate::enums::move_type::MoveType;
use crate::enums::team::Team;

#[derive(Clone)]
pub struct Hare {
    pub team: Option<Team>,
    pub position: u8,
    pub salads: u8,
    pub carrots: u16, 
    pub cards: Vec<Card>,
    // pub last_move: Option<Box<dyn Move>>,
    pub last_move_type: Option<MoveType>
}

impl Hare {
    pub fn new() -> Hare {
        // Create with default values
        Hare {
            team: None,
            position: 0,
            salads: 5,
            carrots: 68,
            cards: Vec::new(),
            // last_move: None,
            last_move_type: None
        }        
    }

    pub fn print(&self) {
        println!("Hare\nTeam: {:?}\nPosition: {}\nSalads: {}\nCarrots: {}\nCards: {:?}\nLast Move Type: {:?}", self.team, self.position, self.salads, self.carrots, self.cards, self.last_move_type);
    }
}