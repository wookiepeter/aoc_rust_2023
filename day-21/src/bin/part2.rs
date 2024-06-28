fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input.to_string()

    // Solution: Divide and conquer

    // The map is not symmetric at all so we have to split everything into the 4 directions
    // For each direction we have to stages -> the filling stage and then the running stage
    // in the filling stage the occupied positions are moving into the quadrant
    // once the quadrants are filled there should be a pattern there that repeats
    // figure out the length of that pattern and then you have the repeating steps

    // So you have to figure out both of these stages for all quadrants and then you can
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process("");
        assert_eq!(result, "4".to_string())
    }
}
