use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut solutions: HashMap<(String, Vec<usize>), u64> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let (first_part, second_part) = line.split_once(' ').unwrap();
            let first_part = vec![first_part.clone(); 5].join("?");
            let second_part = vec![second_part.clone(); 5].join(",");
            let result = day_12::process_line(
                format!("{first_part} {second_part}").as_str(),
                &mut solutions,
            );

            println!("{line} -> {result}");

            result
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "525152".to_string())
    }
}
