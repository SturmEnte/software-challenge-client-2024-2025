pub trait Move {
    fn to_string(&self) -> String; 
}

pub struct AdvanceMove {
    pub distance: i8,
}

impl AdvanceMove {
    pub fn new(distance: i8) -> AdvanceMove {
        AdvanceMove {
            distance: distance,
        }
    }
}

impl Move for AdvanceMove {
    fn to_string(&self) -> String {
        format!("<data class=\"advance\" distance=\"{}\"/>", self.distance)
    }
}

// Old code befor the docs were updated
// use crate::structs::action::Action;

// pub struct Move {
//     pub actions: Vec<Box<dyn Action>>,
// }

// impl Move {
//     pub fn new(actions: Vec<Box<dyn Action>>) -> Move {
//         Move {
//             actions: actions,
//         }
//     }
// }

// Old action file:
/*pub trait Action {
    fn to_string(&self, index: &i8) -> String; 
}

pub struct Advance {
    pub distance: i8,
}

impl Advance {
    pub fn new(distance: i8) -> Advance {
        Advance {
            distance: distance,
        }
    }
}

impl Action for Advance {
    fn to_string(&self, index: &i8) -> String {
        format!("<advance order=\"{}\" distance=\"{}\"/>", index, self.distance)
    }
}*/