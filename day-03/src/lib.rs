use std::ops::Range;

pub fn find_numbers_in_line(line: &str) -> Vec<(Range<usize>, u32)> {
    let mut number_ranges: Vec<(Range<usize>, u32)> = vec![];

    let mut current_number = String::new();
    for (i, c) in line.char_indices() {
        match c {
            c if c.is_numeric() => current_number.push(c),
            _ => {
                if !current_number.is_empty() {
                    number_ranges.push((
                        i.saturating_sub(current_number.len())..(i),
                        current_number.parse::<u32>().unwrap(),
                    ));
                    current_number.clear();
                }
            }
        }
    }

    // case for a number right at the end of the string
    if !current_number.is_empty() {
        number_ranges.push((
            line.len().saturating_sub(current_number.len())..(line.len()),
            current_number.parse::<u32>().unwrap(),
        ));
    }

    number_ranges
}

pub fn check_range_for_symbol(line: &str) -> bool {
    line.chars().any(|c| !c.is_numeric() && c != '.')
}
