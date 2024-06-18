use std::{collections::HashMap, iter};

pub fn build_map(input: &str) -> HashMap<String, Module> {
    let mut map = HashMap::new();

    for line in input.lines() {
        if let Some((left, receivers)) = line.split_once(" -> ") {
            let (first_char, remainder) = left.split_at(1);
            let (mod_type, name) = match first_char {
                "%" => (Type::FlipFlop, remainder.to_string()),
                "&" => (Type::Conjunction, remainder.to_string()),
                start => (Type::Broadcaster, format!("{}{}", start, remainder)),
            };

            let receivers: Vec<String> =
                receivers.split(',').map(|s| s.trim().to_string()).collect();

            map.insert(
                name,
                Module {
                    receivers,
                    flip_state: false,
                    conjunction_state: HashMap::new(),
                    mod_type,
                },
            );
        }
    }

    // Iterator throuhg all Conjunctions and find all the states pointing to them
    let conjunctions: Vec<String> = map
        .iter()
        .filter(|(_, module)| module.mod_type == Type::Conjunction)
        .map(|(name, _)| name.to_string())
        .collect();

    for conjunction in conjunctions {
        let conjunction_state: HashMap<String, bool> = map
            .iter()
            .filter(|(_, module)| module.receivers.contains(&conjunction))
            .map(|(name, _)| name.to_string())
            .zip(iter::repeat(false))
            .collect();

        map.get_mut(&conjunction).unwrap().conjunction_state = conjunction_state;
    }
    map
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub receivers: Vec<String>,
    pub flip_state: bool,
    // should probably be optional but i'm lazy
    pub conjunction_state: HashMap<String, bool>,
    pub mod_type: Type,
}
