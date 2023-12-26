fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let blocks = aoc_util::string_helper::parse_line_blocks(input);
    blocks
        .iter()
        .map(|block| process_block(block.as_str()))
        .sum::<usize>()
        .to_string()
}

fn process_block(block: &str) -> usize {
    let lines: Vec<String> = block.lines().map(String::from).collect();
    let columns: Vec<String> = aoc_util::string_helper::columns_to_lines(block);

    let line_smudge = find_reflection_with_smudge(&lines).unwrap_or(0);
    let column_smudge = find_reflection_with_smudge(&columns).unwrap_or(0);

    column_smudge + 100 * line_smudge
}

fn find_reflection_with_smudge(vec: &Vec<String>) -> Option<usize> {
    (1..vec.len()).find(|prev_lines| {
        let (above_iter, below_iter): (Vec<char>, Vec<char>) = (0..*prev_lines)
            .rev()
            .zip(*prev_lines..vec.len())
            .flat_map(|(above, below)| vec[above].chars().zip(vec[below].chars()))
            .unzip();

        let mut smudge_count = 0;
        for (above, below) in above_iter.iter().zip(below_iter.iter()) {
            if above != below {
                smudge_count += 1;
                if smudge_count > 1 {
                    return false;
                }
            }
        }

        smudge_count == 1
    })
}

#[cfg(test)]
mod tests {
    use crate::process;

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
        assert_eq!(result, "400".to_string())
    }
}
