fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut hands: Vec<day_07::Hand> = input.lines().map(day_07::Hand::from).collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.betted_funds)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "6440".to_string())
    }
}
