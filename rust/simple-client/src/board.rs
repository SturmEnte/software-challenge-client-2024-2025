use crate::field_type::FieldType;

pub struct Board {
	pub board: [FieldType; 34],
	pub initialized: bool,
}

impl Board {
	pub fn new() -> Board {
		Board { board: [FieldType::UNDEFINED; 34], initialized: false }
	}

	pub fn get_field(&self, x: usize) -> FieldType {
		self.board[x]
	}

	pub fn set_field(&mut self, x: usize, value: FieldType) {
		self.board[x] = value;
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
                FieldType::START => print!("🚩"),
                FieldType::CARROTS => print!("🥕"),
                FieldType::HARE => print!("🐇"),
                FieldType::SALAD => print!("🥬"),
                FieldType::MARKET => print!("💹"),
                FieldType::HEDGEHOG => print!("🦔"),
                FieldType::POSITION_1 => print!("1️⃣"),
                FieldType::POSITION_2 => print!("2️⃣"),
                FieldType::GOAL => print!("🏁"),
                FieldType::UNDEFINED => print!("?"),
            }

			print!("\n");
			i += 1;
		}
	}
}