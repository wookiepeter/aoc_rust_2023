use std::{num, ops::Range, thread::current};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    let first_line = sum_up_serial_numbers(day_03::find_numbers_in_line(lines[0]), &lines[0..2]);
    let last_index = lines.len() - 1;
    let last_line = sum_up_serial_numbers(
        day_03::find_numbers_in_line(lines[last_index]),
        &lines[last_index - 1..last_index],
    );
    let main_input: u32 = lines
        .windows(3)
        .map(|window| {
            // Create tuples of all numbers and their ranges.
            let numbers: Vec<(Range<usize>, u32)> = day_03::find_numbers_in_line(window[1]);

            sum_up_serial_numbers(numbers, window)
        })
        .sum();

    (first_line + last_line + main_input).to_string()
}

fn sum_up_serial_numbers(numbers: Vec<(Range<usize>, u32)>, window: &[&str]) -> u32 {
    numbers
        .iter()
        .filter(|(range, number)| {
            println!("Testing for number {number}");
            let test_range =
                range.start.saturating_sub(1)..std::cmp::min(window[0].len(), range.end + 1);
            dbg!(&range);
            dbg!(&test_range);

            window
                .iter()
                .any(|line| day_03::check_range_for_symbol(&line[test_range.clone()]))
        })
        .map(|(_, number)| number)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361".to_string())
    }

    #[test]
    fn test_special_characters() {
        let result = process(
            ".....
.123.
./...",
        );
        assert_eq!(result, "123".to_string())
    }

    #[test]
    fn test_end_of_line() {
        let result = process(
            "....
.225
./..",
        );
        assert_eq!(result, "225".to_string())
    }
}
