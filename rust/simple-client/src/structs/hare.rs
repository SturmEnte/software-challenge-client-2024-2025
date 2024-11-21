use crate::enums::team::Team;

pub struct Hare {
    pub team: Option<Team>,
    pub position: u8,
    pub salads: u8,
    pub carrots: u8, 
}

impl Hare {
    pub fn new() -> Hare {
        Hare {
            team: None,
            position: 0,
            salads: 0,
            carrots: 0,
        }        
    }
}