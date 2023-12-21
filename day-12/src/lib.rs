pub fn process_line(line: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::{generate_solutions, parse_line};

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
}
