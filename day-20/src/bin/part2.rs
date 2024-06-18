use std::collections::{HashMap, HashSet, LinkedList};

use day_20::{build_map, Module, Type};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let map = build_map(input);

    // build a graph using https://mermaid.js.org/ to visualize this problem
    // It consists of 4 secluded subgraphs all connecting in a conjunction on the node "mg" which points directly to "rx"
    // To solve this problem:
    // build those subgraphs
    // find out when all of them sent high pulses at the as the last pulse
    // this should be when "mg" sends a low pulse further

    let subgraphs: HashMap<String, HashMap<String, Module>> = map
        .get("mg")
        .unwrap()
        .conjunction_state
        .keys()
        .map(|mod_name| build_subgraph_map(mod_name.clone(), &map))
        .collect();

    let first_lowpulses: Vec<usize> = subgraphs
        .into_iter()
        .map(|(entry_node, graph)| check_subgraph(entry_node, graph))
        .collect();

    println!("First low pulses: {:?}", first_lowpulses);

    let result = aoc_util::math::lcm_vec(first_lowpulses).unwrap();

    result.to_string()
}

fn check_subgraph(entry_node: String, mut map: HashMap<String, Module>) -> usize {
    let mut queue: LinkedList<(String, String, bool)> = LinkedList::new();

    for i in 0..5000 {
        queue.push_back((entry_node.to_string(), "broadcaster".to_string(), false));

        // process the button pushes
        while !queue.is_empty() {
            let (module_name, incoming_module_name, received_pulse) = queue.pop_front().unwrap();

            if module_name.eq("mg") && received_pulse {
                return i + 1;
            }

            let module = match map.get_mut(module_name.as_str()) {
                Some(module) => module,
                None => {
                    // module doesn't exist in the module list -> ignore otherwise.
                    continue;
                }
            };

            let mut recipients = module.receivers.clone();
            let pulse_to_send = match module.mod_type {
                Type::Broadcaster => received_pulse,
                Type::FlipFlop if !received_pulse => {
                    module.flip_state = !module.flip_state;
                    module.flip_state
                }
                Type::FlipFlop => {
                    recipients = vec![];
                    true
                }
                Type::Conjunction => {
                    module
                        .conjunction_state
                        .insert(incoming_module_name.to_string(), received_pulse);
                    let send_value = !module.conjunction_state.values().all(|value| *value);
                    send_value
                }
            };

            for recipient in recipients {
                queue.push_back((recipient.clone(), module_name.clone(), pulse_to_send));
            }
        }
    }
    panic!("Not enough iterations to find solution for subgraph");
}

fn build_subgraph_map(
    end_node: String,
    full_map: &HashMap<String, Module>,
) -> (String, HashMap<String, Module>) {
    let mut entry_node: String = "".to_string();

    let mut subgraph_members: HashSet<&str> = HashSet::new();
    subgraph_members.insert(&end_node);
    let mut queue: LinkedList<String> = LinkedList::new();
    queue.push_back(end_node.clone());

    while let Some(current_node) = queue.pop_front() {
        full_map
            .iter()
            .filter_map(
                |(sending_module, module)| match module.receivers.contains(&current_node) {
                    true => Some(sending_module),
                    false => None,
                },
            )
            .for_each(|sending_module| {
                if sending_module.eq("broadcaster") {
                    entry_node.clone_from(&current_node);
                } else if subgraph_members.insert(sending_module) {
                    queue.push_back(sending_module.to_string());
                }
            })
    }

    let map = subgraph_members
        .into_iter()
        .map(|mod_name| {
            (
                mod_name.to_string(),
                full_map.get(mod_name).unwrap().clone(),
            )
        })
        .collect();

    (entry_node, map)
}
