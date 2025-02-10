use crate::game::field_type::FieldType;

pub fn string_to_field_type(input: String) -> Option<FieldType>{
    match input.as_str() {
        "START" => Some(FieldType::Start),
        "CARROTS" => Some(FieldType::Carrots),
        "SALAD" => Some(FieldType::Salad),
        "POSITION_1" => Some(FieldType::Position1),
        "POSITION_2" => Some(FieldType::Position2),
        "HEDGEHOG" => Some(FieldType::Hedgehog),
        "MARKET" => Some(FieldType::Market),
        "HARE" => Some(FieldType::Hare),
        "GOAL" => Some(FieldType::Goal),
        _ => None,
    }
}