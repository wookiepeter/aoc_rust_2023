pub fn parse_numbers(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, number_str) = line.split_once(':').unwrap();
    let (winning, drawn) = number_str.trim().split_once('|').unwrap();

    let winning = winning
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let drawn = drawn
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    (winning, drawn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_parsing() {
        let result = crate::parse_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            format!("{:?}", result.0),
            format!("{:?}", vec![41, 48, 83, 86, 17])
        );
        assert_eq!(
            format!("{:?}", result.1),
            format!("{:?}", vec![83, 86, 6, 31, 17, 9, 48, 53])
        );
    }
}
