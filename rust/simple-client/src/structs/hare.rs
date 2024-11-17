use crate::enums::team::Team;

pub struct Hare {
    pub team: Option<Team>,
    pub position: i8,
    pub salads: i8,
    pub carrots: i8, 
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