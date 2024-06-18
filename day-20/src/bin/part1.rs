use std::collections::{HashMap, LinkedList};

use day_20::{build_map, Module, Type};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut map = build_map(input);

    println!("{:?}", map);

    let mut result: (u32, u32) = (0, 0);
    let mut queue: LinkedList<(String, String, bool)> = LinkedList::new();

    for _ in 0..1000 {
        queue.push_back(("broadcaster".to_string(), "button".to_string(), false));

        // process the button pushes
        while !queue.is_empty() {
            let (module_name, incoming_module_name, received_pulse) = queue.pop_front().unwrap();

            match received_pulse {
                true => {
                    result = (result.0 + 1, result.1);
                }
                false => {
                    result = (result.0, result.1 + 1);
                }
            };

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
                    !module.conjunction_state.values().all(|value| *value)
                }
            };
            for recipient in recipients {
                queue.push_back((recipient.clone(), module_name.clone(), pulse_to_send));
            }
        }
    }

    println!("{:?}", result);

    (result.0 * result.1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, "32000000".to_string())
    }

    #[test]
    fn test_example2() {
        let result = process(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        assert_eq!(result, "11687500".to_string())
    }
}
