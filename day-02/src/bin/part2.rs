use day_02::Cubes;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(day_02::parse_game_input)
        .map(|(_, cubes)| {
            let min_red = cubes
                .iter()
                .filter(|cube| matches!(*cube, Cubes::Red(_)))
                .map(|cube| match *cube {
                    Cubes::Red(value) => value,
                    _ => 0,
                })
                .max()
                .unwrap();
            let min_green = cubes
                .iter()
                .filter(|cube| matches!(*cube, Cubes::Green(_)))
                .map(|cube| match *cube {
                    Cubes::Green(value) => value,
                    _ => 0,
                })
                .max()
                .unwrap();

            let min_blue = cubes
                .iter()
                .filter(|cube| matches!(*cube, Cubes::Blue(_)))
                .map(|cube| match *cube {
                    Cubes::Blue(value) => value,
                    _ => 0,
                })
                .max()
                .unwrap();
            // println!("Red: {}, Green: {}, Blue: {}", min_red, min_green, min_blue);
            min_red * min_green * min_blue
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string())
    }
}
