use crate::enums::field_type::FieldType;

pub struct Board {
	pub board: [FieldType; crate::FIELD_COUNT],
	pub initialized: bool,
}

impl Board {
	pub fn new() -> Board {
		Board { board: [FieldType::Undefined; crate::FIELD_COUNT], initialized: false }
	}

	pub fn get_field(&self, x: usize) -> FieldType {
		self.board[x]
	}

	pub fn set_field(&mut self, x: usize, value: &str) {
		// self.board[x] = value;

        match value {
            "START" => self.board[x] = FieldType::Start,
            "CARROTS" => self.board[x] = FieldType::Carrots,
            "HARE" => self.board[x] = FieldType::Hare,
            "SALAD" => self.board[x] = FieldType::Salad,
            "MARKET" => self.board[x] = FieldType::Market,
            "HEDGEHOG" => self.board[x] = FieldType::Hedgehog,
            "POSITION_1" => self.board[x] = FieldType::Position1,
            "POSITION_2" => self.board[x] = FieldType::Position2,
            "GOAL" => self.board[x] = FieldType::Goal,
            _ => {
                println!("Unknown field type in position {}: {}", x, value);
                self.board[x] = FieldType::Undefined;
            }
        }
	}

	pub fn print(&self) {
		let mut i: usize = 0;
		for field_type in self.board {
			
			if i == 0 {
                for j in 0..17 {
                    print!("{} ", j);
                }
                print!("\n");
            }

            if i == 18 {
                print!("\n\n");
                for j in 18..33 {
                    print!("{} ", j);
                }
                print!("\n");
            }
            
            match field_type {
                FieldType::Start => print!("🚩"),
                FieldType::Carrots => print!("🥕"),
                FieldType::Hare => print!("🐇"),
                FieldType::Salad => print!("🥬"),
                FieldType::Market => print!("🏪"),
                FieldType::Hedgehog => print!("🦔"),
                FieldType::Position1 => print!("1️⃣"),
                FieldType::Position2 => print!("2️⃣"),
                FieldType::Goal => print!("🏁"),
                FieldType::Undefined => print!("?"),
            }

			print!("\n");
			i += 1;
		}
	}
}