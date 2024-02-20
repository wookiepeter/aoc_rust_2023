pub fn compute_hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| {
        let mut current_value = acc + u32::from(c);
        current_value *= 17;
        current_value % 256
    })
}
