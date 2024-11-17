use crate::enums::field_type::FieldType;

pub struct Board {
	pub board: [Option<FieldType>; crate::FIELD_COUNT],
	pub initialized: bool,
}

impl Board {
	pub fn new() -> Board {
		Board { board: [None; crate::FIELD_COUNT], initialized: false }
	}

	pub fn get_field(&self, x: usize) -> Option<FieldType> {
		self.board[x]
	}

	pub fn set_field(&mut self, x: usize, field_string: &str) {
        match field_string {
            "START" => self.board[x] = Some(FieldType::Start),
            "CARROTS" => self.board[x] = Some(FieldType::Carrots),
            "HARE" => self.board[x] = Some(FieldType::Hare),
            "SALAD" => self.board[x] = Some(FieldType::Salad),
            "MARKET" => self.board[x] = Some(FieldType::Market),
            "HEDGEHOG" => self.board[x] = Some(FieldType::Hedgehog),
            "POSITION_1" => self.board[x] = Some(FieldType::Position1),
            "POSITION_2" => self.board[x] = Some(FieldType::Position2),
            "GOAL" => self.board[x] = Some(FieldType::Goal),
            _ => {
                println!("Unknown field type in position {}: {}", x, field_string);
            }
        }
	}

	pub fn print(&self) {
		
        for i in 0..crate::FIELD_COUNT {
            print!("{:2} ", i);
        }

        // End the field number line
        println!("");
        
		for field_type in self.board {
            
            // Get the emoji coresponding to the current fields type
            let emoji: &str;

            if field_type.is_some() {
                match field_type.unwrap() {
                    FieldType::Start => emoji = "🚩",
                    FieldType::Carrots => emoji = "🥕",
                    FieldType::Hare => emoji = "🐇",
                    FieldType::Salad => emoji = "🥬",
                    FieldType::Market => emoji = "🏪",
                    FieldType::Hedgehog => emoji = "🦔",
                    FieldType::Position1 => emoji = "1️⃣ ",
                    FieldType::Position2 => emoji = "2️⃣ ",
                    FieldType::Goal => emoji = "🏁",
                }
            } else {
                emoji = "?";
            }
            
            // Print the emoji
            print!("{} ", emoji);
		}

        // End the emoji line
        println!("");
	}
}