use num::*;
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

    let mut current_vec: Vec<&str> = line_dict
        .keys()
        .filter_map(|key| match key.ends_with('A') {
            true => Some(*key),
            false => None,
        })
        .collect();

    let mut cycle_length: Vec<usize> = vec![];

    // follow the LR-instructions and find length of the cycle for each of the starting elements
    for (count, c) in directions.chars().cycle().enumerate() {
        let cycle_end: Vec<usize> = current_vec
            .iter()
            .enumerate()
            .filter(|(_, value)| value.ends_with('Z'))
            .map(|(index, _)| index)
            .collect();

        cycle_end.iter().for_each(|index| {
            cycle_length.push(count);
            _ = current_vec.remove(*index)
        });

        if current_vec.is_empty() {
            break;
        }

        match c {
            'R' => {
                current_vec = current_vec
                    .iter()
                    .map(|key| line_dict.get(key).unwrap().1)
                    .collect::<Vec<&str>>();
            }
            'L' => {
                current_vec = current_vec
                    .iter()
                    .map(|key| line_dict.get(key).unwrap().0)
                    .collect::<Vec<&str>>();
            }
            _ => panic!("Direction should be either left or right"),
        }
    }

    // compute greatest common multiplicator for all cycles -> should be the first cycle match
    cycle_length
        .iter()
        .fold(1u64, |acc, e| acc.lcm(&(*e as u64)))
        .to_string()
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
    fn simple_test() {
        let result = process(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string())
    }
}
