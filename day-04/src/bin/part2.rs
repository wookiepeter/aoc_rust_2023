fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let matching_cards: Vec<u32> = input
        .lines()
        .map(day_04::parse_numbers)
        .map(|(winners, drawn)| {
            let count = drawn.iter().filter(|num| winners.contains(num)).count() as u32;
            count
        })
        .collect();
    let mut card_numbers = vec![1; matching_cards.len()];

    let mut sum = 0;
    for (index, card_count) in matching_cards.iter().enumerate() {
        sum += card_numbers[index];
        let range = (index + 1)
            ..usize::min(
                card_numbers.len(),
                index + usize::try_from(*card_count + 1).unwrap(),
            );

        for i in range {
            card_numbers[i] += card_numbers[index];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30".to_string())
    }
}
