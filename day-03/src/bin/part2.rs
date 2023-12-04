use std::ops::Range;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let filtered_numbers: Vec<Vec<(Range<usize>, u32)>> =
        input.lines().map(day_03::find_numbers_in_line).collect();

    let mut sum = 0;
    for (row, line) in input.lines().enumerate() {
        for (index, _) in line.char_indices().filter(|(_, c)| *c == '*') {
            let vec: Vec<u32> = filtered_numbers[row.saturating_sub(1)..row.saturating_add(2)]
                .iter()
                .flatten()
                .filter(|(range, _)| {
                    let my_range = range.start.saturating_sub(1)..range.end.saturating_add(1);
                    my_range.contains(&index)
                })
                .map(|(_, num)| *num)
                .collect();

            if vec.len() == 2 {
                sum += vec.iter().product::<u32>();
            }
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
        assert_eq!(result, "467835".to_string())
    }

    #[test]
    fn test_base_case() {
        let result = process(
            ".....
.100.
..*..
..23.
.....",
        );
        assert_eq!(result, "2300".to_string())
    }
}
