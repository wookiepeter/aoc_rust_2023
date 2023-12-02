pub enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

pub fn parse_game_input(line: &str) -> (u32, Vec<Cubes>) {
    let (game_id, draws) = line.split_once(':').unwrap();

    let mut cubes = vec![];

    draws.split(';').for_each(|game| {
        game.split(',').for_each(|cube_info| {
            let (count, color) = cube_info.trim().split_once(' ').unwrap();

            cubes.push(match color {
                "red" => Cubes::Red(count.parse::<u32>().unwrap()),
                "green" => Cubes::Green(count.parse::<u32>().unwrap()),
                "blue" => Cubes::Blue(count.parse::<u32>().unwrap()),
                _ => panic!("Should have found a color here!"),
            })
        })
    });
    (
        game_id.split_once(' ').unwrap().1.parse::<u32>().unwrap(),
        cubes,
    )
}
