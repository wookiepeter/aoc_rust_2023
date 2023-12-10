use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut iter = input.lines();

    let directions = iter.next().unwrap();
    // skip empty line
    _ = iter.next();

    let line_dict = iter
        .map(parse_line)
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut current = "AAA";
    for (count, c) in directions.chars().cycle().enumerate() {
        if current == "ZZZ" {
            return count.to_string();
        }
        current = match c {
            'R' => line_dict.get(current).unwrap().1,
            'L' => line_dict.get(current).unwrap().0,
            _ => panic!("Direction should be either left or right"),
        }
    }
    panic!("Loop should always have elements!")
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let (key, remainder) = line.split_once(" = ").unwrap();
    let (left, right) = remainder
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(", ")
        .unwrap();

    (key, (left, right))
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn short_instructio_test() {
        let result = process(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2".to_string())
    }

    #[test]
    fn cycling_instruction_test() {
        let result = process(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ))",
        );
        assert_eq!(result, "6".to_string())
    }
}
