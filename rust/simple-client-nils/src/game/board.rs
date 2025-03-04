use std::fmt::Display;

use super::field_type::FieldType;

pub const HEDGEHOG_FIELDS: [u8; 9] = [11, 15, 19, 24, 30, 37, 43, 50, 56];
pub const SALAD_FIELDS: [u8; 4] = [10, 22, 42, 57];
pub const FIRST_HEADGEHOG: u8 = 11;
pub const LAST_SALAD: u8 = 57;

#[derive(Clone)]
pub struct Board {
    pub board: [FieldType; 65]
}

impl Board {
    pub fn new(board: [FieldType; 65]) -> Self {
        Board {
            board: board
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board: [")?;
        for (i, field) in self.board.iter().enumerate() {
            write!(f, "[{}|{}]", i, field)?;
        }
        write!(f, "]\n")?;
        Ok(())
    } 

}