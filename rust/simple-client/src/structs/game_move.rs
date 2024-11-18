use crate::structs::action::Action;

pub struct Move {
    pub actions: Vec<Box<dyn Action>>,
}