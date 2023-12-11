fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut sequence = day_09::Sequence::from(line);
            sequence.compute_backward_sequence();
            sequence.get_first_value()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "2".to_string())
    }
}
