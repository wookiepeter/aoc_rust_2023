use aoc_util::string_grid::*;
use aoc_util::usize_point::Point;
use std::collections::{HashMap, HashSet};

use day_16::{process_beam, MapData, MirrorStatus};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    /*
    Rough Idea

    HashMap die alle Positionen von Spiegeln enthält
        Key: (usize, usize), Value: Enum (Fully Hit, HitLeft, HitRight)
    HashSet mit allen Positionen die energyzed sind

    Main loop:
    - depth search for open positions
    - hand mutable references to the deciding data structures
    - hitting a mirror -> evaluate if it's hit
        if yes -> do nothing
        if no -> evaluate each direction
        set the appropriate calls
     */
    let char_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map_size = (char_map[0].len(), char_map.len());

    let map_data = MapData {
        chars: char_map.clone(),
        size: map_size,
    };

    let mut mirror_map: HashMap<(usize, usize), MirrorStatus> = HashMap::new();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();

    let position: Point = (0, 0);
    let direction: (i32, i32) = (1, 0);

    process_beam(
        position,
        direction,
        &map_data,
        &mut energized,
        &mut mirror_map,
    );

    // Used for visual debugging!
    /*
    let dash_list = char_map
        .iter()
        .enumerate()
        .flat_map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(|(index, c)| match c {
                    '-' => Some((index, line)),
                    _ => None,
                })
                .collect::<Vec<Point>>()
        })
        .collect();
    let pipe_list = char_map
        .iter()
        .enumerate()
        .flat_map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(|(index, c)| match c {
                    '|' => Some((index, line)),
                    _ => None,
                })
                .collect::<Vec<Point>>()
        })
        .collect();
    let slash_list = char_map
        .iter()
        .enumerate()
        .flat_map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(|(index, c)| match c {
                    '/' => Some((index, line)),
                    _ => None,
                })
                .collect::<Vec<Point>>()
        })
        .collect();
    let backslash_list = char_map
        .iter()
        .enumerate()
        .flat_map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(|(index, c)| match c {
                    '\\' => Some((index, line)),
                    _ => None,
                })
                .collect::<Vec<Point>>()
        })
        .collect();

    let grid_info = GridInfo {
        default_char: '.',
        dimensions: map_data.size,
        other_chars: vec![
            (
                'o',
                energized.iter().copied().collect::<Vec<(usize, usize)>>(),
            ),
            ('-', dash_list),
            ('|', pipe_list),
            ('/', slash_list),
            ('\\', backslash_list),
        ],
    };

    println!("{}", grid_info.create_grid());

    println!("++++++++++++++++++++++++");

    let grid_info = GridInfo {
        default_char: '.',
        dimensions: map_data.size,
        other_chars: vec![(
            'o',
            energized.iter().copied().collect::<Vec<(usize, usize)>>(),
        )],
    };

    println!("{}", grid_info.create_grid());
    */

    energized.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, "46".to_string())
    }

    #[test]
    fn case_test_1() {
        let result = process(
            r"/..
...
...",
        );
        assert_eq!(result, "1".to_string())
    }

    #[test]
    fn case_test_2() {
        let result = process(
            r"\..
...
...",
        );
        assert_eq!(result, "3".to_string())
    }

    #[test]
    fn case_test_3() {
        let result = process(
            r"-..
...
...",
        );
        assert_eq!(result, "3".to_string())
    }

    #[test]
    fn case_test_4() {
        let result = process(
            r"|..
...
...",
        );
        assert_eq!(result, "3".to_string())
    }

    #[test]
    fn case_test_5() {
        let result = process(
            r"\..
.\\
\./",
        );
        assert_eq!(result, "8".to_string())
    }

    #[test]
    fn case_test_6() {
        let result = process(
            r".|.
.-|
...
",
        );
        assert_eq!(result, "7".to_string())
    }

    #[test]
    fn case_test_7() {
        let result = process(
            r"...\...
.......
-......
.......
\../...",
        );
        assert_eq!(result, "18".to_string())
    }

    #[test]
    fn case_test_8() {
        let result = process(
            r"|....-
......
......
-....|",
        );
        assert_eq!(result, "16".to_string())
    }

    #[test]
    fn case_test_9() {
        let result = process(
            r"\........-.........\................................|...
......-/.............|-.../.....|...........././..\.....
-.........................|.....\...................|.\.
.......-........../.......\.........|..../........-.-|..",
        );
        assert_eq!(result, "89".to_string())
    }

    #[test]
    fn case_test_escaped_slash() {
        let result = process(
            r"..|..
.....
./\..
.....",
        );
        assert_eq!(result, "7".to_string())
    }

    #[test]
    fn case_test_escaped_slash_second_beam() {
        let result = process(
            r"\.|..\
......
./\...
\/-..|",
        );
        assert_eq!(result, "20".to_string())
    }
}
