use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    const RADIX: u32 = 10;

    input
        .lines()
        .map(|line: &str| {
            // Build indices with all the first and last occurances the possible latters
            let mut digits: Vec<(usize, u32)> = vec![];

            for (key, value) in numbers.iter() {
                if let Some(index) = line.find(key) {
                    digits.push((index, *value))
                }
                if let Some(index) = line.rfind(key) {
                    digits.push((index, *value));
                }
                if let Some(index) = line.find(|c: char| c.is_numeric()) {
                    digits.push((
                        index,
                        line.chars().nth(index).unwrap().to_digit(RADIX).unwrap(),
                    ));
                }
                if let Some(index) = line.rfind(|c: char| c.is_numeric()) {
                    digits.push((
                        index,
                        line.chars().nth(index).unwrap().to_digit(RADIX).unwrap(),
                    ));
                }
            }
            digits.sort_by_key(|value| value.0);
            let result = format!("{}{}", digits.first().unwrap().1, digits.last().unwrap().1)
                .parse::<u32>()
                .unwrap();

            println!("{}", result);

            result
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example2() {
        let result = process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string())
    }
}
