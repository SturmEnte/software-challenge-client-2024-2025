use super::field_type::FieldType;

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