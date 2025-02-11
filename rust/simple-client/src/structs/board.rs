use crate::enums::field_type::FieldType;

pub struct Board {
	pub board: [Option<FieldType>; crate::FIELD_COUNT],
}

impl Board {
	pub fn new() -> Board {
		Board { board: [None; crate::FIELD_COUNT] }
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

            match field_type {
                Some(FieldType::Start) => emoji = "🚩",
                Some(FieldType::Carrots) => emoji = "🥕",
                Some(FieldType::Hare) => emoji = "🐇",
                Some(FieldType::Salad) => emoji = "🥬",
                Some(FieldType::Market) => emoji = "🏪",
                Some(FieldType::Hedgehog) => emoji = "🦔",
                Some(FieldType::Position1) => emoji = "1️⃣ ",
                Some(FieldType::Position2) => emoji = "2️⃣ ",
                Some(FieldType::Goal) => emoji = "🏁",
                None => emoji = "?"
            }
            
            // Print the emoji
            print!("{} ", emoji);
		}

        // End the emoji line
        println!("");
	}
}