const HEDGEHOG_FIELDS: [u8; 9] = [11, 15, 19, 24, 30, 37, 43, 50, 56];

// Returns the nearest hedgehog field to the hare's position
// 0 means that the hare is not near any hedgehog field
pub fn get_nearest_hedgehog_field(hare_position: &u8) -> Option<&u8> {
    for field in HEDGEHOG_FIELDS.iter().rev() {
        if hare_position > field {
            return Some(field);
        }
    }
    
    None
}