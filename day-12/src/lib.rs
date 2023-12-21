use std::{collections::HashMap, result};

pub fn process_line_unoptimized(line: &str) -> usize {
    let (line, vec) = parse_line(line);
    generate_solutions(line, &vec)
}

pub fn generate_solutions(line: String, groups: &Vec<usize>) -> usize {
    if let Some(index) = line.find('?') {
        let mut left_string = line.clone();
        left_string.replace_range(index..index + 1, ".");
        let mut right_string = line.clone();
        right_string.replace_range(index..index + 1, "#");
        let lhs = generate_solutions(left_string, groups);
        let rhs = generate_solutions(right_string, groups);
        rhs + lhs
    } else {
        match check_if_group_fits(line.as_str(), groups) {
            true => 1,
            false => 0,
        }
    }
}

fn check_if_group_fits(line: &str, groups: &Vec<usize>) -> bool {
    let iter = line.chars();

    let mut cur_group_size = 0;
    let mut cur_group_index = 0;
    for c in iter {
        if c == '#' {
            cur_group_size += 1;
        } else if cur_group_size > 0 {
            if cur_group_index >= groups.len() || groups[cur_group_index] != cur_group_size {
                return false;
            }
            cur_group_index += 1;
            cur_group_size = 0;
        }
    }

    if cur_group_size > 0 {
        if cur_group_index <= (groups.len() - 1) && groups[cur_group_index] == cur_group_size {
            cur_group_index += 1;
        } else {
            return false;
        }
    }

    cur_group_index == groups.len()
}

pub fn parse_line(line: &str) -> (String, Vec<usize>) {
    let (conditions, groups) = line.split_once(' ').unwrap();

    (
        conditions.to_string(),
        groups
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

///// Optimized implementation for both parts /////
/*
Analyze string left to right

if it starts with a ., discard the . and recursively check again.

if it starts with a ?, replace the ? with a . and recursively check again, AND replace it with a # and recursively check again.

it it starts with a #, check if it is long enough for the first group, check if all characters in the first [grouplength] characters are not '.', and then remove the first [grouplength] chars and the first group number, recursively check again.

at some point you will get to the point of having an empty string and more groups to do - that is a zero. or you have an empty string with zero gropus to do - that is a one.

there are more rules to check than these few, which are up to you to find. but this is a way to work out the solution.

*/
pub fn process_line(
    line: &str,
    previous_solutions: &mut HashMap<(String, Vec<usize>), u64>,
) -> u64 {
    let (line, vec) = parse_line(line);
    process_recursive(&line, vec, previous_solutions)
}

fn process_recursive(
    line: &str,
    groups: Vec<usize>,
    previous_solutions: &mut HashMap<(String, Vec<usize>), u64>,
) -> u64 {
    if let Some(result) = previous_solutions.get(&(line.to_string(), groups.clone())) {
        return *result;
    }
    if line.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    let mut result = 0;
    if line.starts_with('.') {
        result = process_recursive(
            line.replacen('.', "", 1).as_str(),
            groups.clone(),
            previous_solutions,
        );
    } else if line.starts_with('?') {
        result = process_recursive(
            line.replacen('?', ".", 1).as_str(),
            groups.clone(),
            previous_solutions,
        );

        result += process_recursive(
            line.replacen('?', "#", 1).as_str(),
            groups.clone(),
            previous_solutions,
        )
    } else if line.starts_with('#') {
        if groups.is_empty() || line.len() < groups[0] || line[0..groups[0]].contains('.') {
            return 0;
        } else if line.len() == groups[0] {
            let mut new_group = groups.clone();
            new_group.remove(0);
            result = process_recursive("", new_group, previous_solutions);
        } else if line[groups[0]..groups[0] + 1].eq("#") {
            return 0;
        } else {
            let mut new_group = groups.clone();
            let group_len = new_group.remove(0);
            let new_line = line[(group_len + 1)..].to_string();
            result = process_recursive(new_line.as_str(), new_group.clone(), previous_solutions);
            previous_solutions.insert((new_line.to_string(), new_group.clone()), result);
        }
    }

    // println!("{line} + {:?} -> {result}", groups);

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{generate_solutions, parse_line, process_line};

    #[test]
    fn test_example() {
        let (line, groups) = parse_line(".??..??...?##. 1,1,3");

        let result = generate_solutions(line, &groups);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        let (line, groups) = parse_line("???.### 1,1,3");

        let result = generate_solutions(line, &groups);
        assert_eq!(result, 1);
    }

    #[test]
    fn complicated_test() {
        let (line, groups) = parse_line("?###???????? 3,2,1");

        let result = generate_solutions(line, &groups);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_optimized_recursive() {
        let line = ".??..??...?##. 1,1,3";

        let mut solutions: HashMap<(String, Vec<usize>), u64> = HashMap::new();

        let result = process_line(line, &mut solutions);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test_optimized_recursive() {
        let line = "?###???????? 3,2,1";

        let mut solutions: HashMap<(String, Vec<usize>), u64> = HashMap::new();

        let result = process_line(line, &mut solutions);
        assert_eq!(result, 10);
    }
}
