type Lens = (String, usize);

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    input
        .split(',')
        .for_each(|step| handle_step(step, &mut boxes));

    compute_focusing_power(&boxes).to_string()
}

fn handle_step(command: &str, boxes: &mut [Vec<Lens>]) {
    let operation_index = command.find(|c| c == '=' || c == '-').unwrap();
    let label = command.get(0..operation_index).unwrap();
    let operation = command.get(operation_index..(operation_index + 1)).unwrap();

    let box_index: usize = day_15::compute_hash(label) as usize;
    let existing_element = boxes[box_index].iter().position(|lens| lens.0 == label);

    match operation {
        "=" => {
            let focal_length = command
                .get((operation_index + 1)..)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            match existing_element {
                Some(index) => boxes[box_index][index] = (label.to_string(), focal_length),
                None => boxes[box_index].push((label.to_string(), focal_length)),
            };
        }
        "-" => {
            if let Some(index) = existing_element {
                boxes[box_index].remove(index);
            }
        }
        _ => panic!("Command {} does not contain a valid operation", command),
    };
}

fn compute_focusing_power(boxes: &[Vec<Lens>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| (box_index + 1) * (lens_index + 1) * lens.1)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145".to_string())
    }
}
