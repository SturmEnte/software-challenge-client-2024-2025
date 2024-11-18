use crate::structs::action::Action;

pub struct Move {
    pub actions: Vec<Box<dyn Action>>,
}

impl Move {
    pub fn new(actions: Vec<Box<dyn Action>>) -> Move {
        Move {
            actions: actions,
        }
    }
}