pub fn calculate_triangular_number(n: u16) -> u16 {
    n * (n + 1) /2
}

pub fn calculate_reverse_triangular_number_floor(n: u16) -> u16 {
    (((n * 8) as f64 - 1.0).sqrt() / 2.0).floor() as u16
}