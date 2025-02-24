use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Start,
    Carrots,
    Salad,
    Position1,
    Position2,
    Hedgehog,
    Market,
    Hare,
    Goal,
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Start => write!(f, " 🏳️‍⚧️"),
            FieldType::Carrots => write!(f, " 🥕"),
            FieldType::Salad => write!(f, " 🥬"),
            FieldType::Position1 => write!(f, " 1️⃣"),
            FieldType::Position2 => write!(f, " 2️⃣"),
            FieldType::Hedgehog => write!(f, " 🦔"),
            FieldType::Market => write!(f, " 🏪"),
            FieldType::Hare => write!(f, " 🐰"),
            FieldType::Goal => write!(f, " 🏁"),
        }
    }
}