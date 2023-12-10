fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut iter = input.lines();
    let time = iter
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = iter
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    compute_possible_results(&time, &distance).to_string()
}

pub fn compute_possible_results(time: &u64, distance_to_beat: &u64) -> u64 {
    let first_match = (1..*time)
        .find(|num| (time - num) * num > *distance_to_beat)
        .unwrap();

    let last_match = (1..*time)
        .rev()
        .find(|num| (time - num) * num > *distance_to_beat)
        .unwrap();

    last_match - first_match + 1
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string())
    }
}
