fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let line_blocks = aoc_util::string_helper::parse_line_blocks(input);

    line_blocks
        .iter()
        .map(process_line_block)
        .sum::<usize>()
        .to_string()
}

fn process_line_block(block: &String) -> usize {
    let line_vec = block
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let line_reflection_value = calculate_reflection_count(line_vec);

    let column_vec = aoc_util::string_helper::columns_to_lines(&block);
    let column_reflection_value = calculate_reflection_count(column_vec);

    // println!("columns: {column_reflection_value} + lines: {line_reflection_value}");

    column_reflection_value + 100 * line_reflection_value
}

/// check for any posibble reflection line if all the strings above match the ones below
/// and sum up the number of lines above all reflections lines for the result
fn calculate_reflection_count(vec: Vec<String>) -> usize {
    (1..vec.len())
        .filter(|prev_lines| {
            (0..*prev_lines)
                .rev()
                .zip(*prev_lines..vec.len())
                .all(|(above, below)| vec[above].eq(&vec[below]))
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_reflection_count, process};

    #[test]
    fn test_example() {
        let result = process(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, "405".to_string())
    }

    #[test]
    fn test_reflection_count() {
        let result = calculate_reflection_count(vec![
            "0100".to_string(),
            "0100".to_string(),
            "0100".to_string(),
        ]);

        assert_eq!(result, 3)
    }
}
