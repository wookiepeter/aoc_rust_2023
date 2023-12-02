pub fn get_digit_from_line(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
    let second_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap();

    let result = format!("{first_digit}{second_digit}")
        .parse::<u32>()
        .unwrap();

    println!("{}", result);

    result
}
