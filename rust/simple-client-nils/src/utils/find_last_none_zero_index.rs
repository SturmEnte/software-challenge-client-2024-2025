pub fn find_last_non_zero_index(buffer: &[u8]) -> usize {
    return buffer.iter().rposition(|&x| x != 0).unwrap_or(0);
}