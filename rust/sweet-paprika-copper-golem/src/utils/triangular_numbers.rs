pub fn calculate_triangular_number(n: u16) -> u16 {
    n * (n + 1) /2
}

pub fn calculate_reverse_triangular_number_floor(n: u16) -> u16 {
    return ((-1.0 + ((1.0 + 8.0 * n as f64).sqrt())) / 2.0).floor() as u16;
}