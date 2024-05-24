use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone)]
pub enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

pub struct Rule {
    /// Insted of names (x, m, a, s) I use Vecs to store the 4 values therefore index!
    pub tested_index: usize,
    pub ordering: std::cmp::Ordering,
    /// value that is being tested / compared
    pub value: usize,
    pub destination: Destination,
}

impl Rule {
    /// Returns Destination if rule fits to part
    pub fn process_rule(&self, part: &[usize]) -> Option<&Destination> {
        match part[self.tested_index].cmp(&self.value) == self.ordering {
            true => Some(&self.destination),
            false => None,
        }
    }
}

pub fn process_rules(rule_string: Vec<&str>) -> HashMap<String, (Vec<Rule>, Destination)> {
    rule_string
        .iter()
        .map(|line| {
            let mut iter = line.split('{').rev();
            let rules = iter.by_ref().next().unwrap().trim_end_matches('}');
            let rule_vec: Vec<&str> = rules.split(',').collect();
            let last_index = rule_vec.len() - 1;
            let rules = rule_vec[0..last_index]
                .iter()
                .cloned()
                .map(parse_rule)
                .collect();

            let destination = match rule_vec[last_index] {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                dest => Destination::Workflow(dest.to_string()),
            };
            (iter.next().unwrap().to_string(), (rules, destination))
        })
        .collect()
}

pub fn parse_rule(rule: &str) -> Rule {
    let mut split = rule.split(':');
    let mut condition = split.by_ref().next().unwrap().chars();
    let tested_index: usize = match condition.next().unwrap() {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        other_char => panic!("Should not happen, char: {}, rule: {}", other_char, rule),
    };

    let ordering = match condition.next().unwrap() {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => panic!("Should always be Less or Greater!"),
    };

    let value_string: String = condition.collect();
    let value = value_string.parse::<usize>().unwrap();

    let destination = match split.next().unwrap() {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        dest => Destination::Workflow(dest.to_string()),
    };

    Rule {
        tested_index,
        ordering,
        value,
        destination,
    }
}
