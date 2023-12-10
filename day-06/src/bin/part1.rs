fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut iter = input.lines();
    let times = iter
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|dist| dist.parse::<u32>().unwrap());
    let distances = iter
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|dist| dist.parse::<u32>().unwrap());

    let races: Vec<(u32, u32)> = times.zip(distances).collect();

    races
        .iter()
        .map(|(time, distance_to_beat)| -> u32 { compute_possible_results(time, distance_to_beat) })
        .product::<u32>()
        .to_string()
}

pub fn compute_possible_results(time: &u32, distance_to_beat: &u32) -> u32 {
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
        assert_eq!(result, "288".to_string())
    }
}
