fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (first_part, second_part) = line.split_once(' ').unwrap();
            let first_part = vec![first_part.clone(); 5].join("?");
            let second_part = vec![second_part.clone(); 5].join("");
            let result = day_12::process_line(format!("{first_part} {second_part}").as_str());

            result as u64
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
        assert_eq!(result, "506250".to_string())
    }
}
