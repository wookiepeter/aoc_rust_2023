use std::{cmp::Ordering, collections::VecDeque, ops::Range};

use day_19::{process_rules, Destination, Rule};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

const LOWER_BOUND: usize = 1;
const UPPER_BOUND: usize = 4001;

fn process(input: &str) -> String {
    let mut iter = input.lines();
    let rule_string: Vec<&str> = iter.by_ref().take_while(|line| !line.is_empty()).collect();

    let rules = process_rules(rule_string);
    let mut accepted_ranges: Vec<Vec<Range<usize>>> = vec![];

    let mut queue: VecDeque<PartRange> = VecDeque::new();
    queue.push_back(PartRange {
        ranges: vec![LOWER_BOUND..UPPER_BOUND; 4],
        workflow: "in".to_string(),
    });

    while let Some(queue_range) = queue.pop_front() {
        let mut current = queue_range.clone();
        // process current range and queue any potential children
        let (rules, default) = rules.get(&current.workflow).unwrap();
        for rule in rules {
            // check if part can match rule or no
            // if not -> ignore and move on
            let affected_range: Range<usize> = current.ranges[rule.tested_index].clone();
            let (accepted_range, remaining_range) = find_matching_range(&affected_range, rule);

            match &rule.destination {
                Destination::Accept => {
                    current.update_range(rule.tested_index, accepted_range);
                    accepted_ranges.push(current.ranges.clone());
                }
                Destination::Reject => (),
                Destination::Workflow(id) => {
                    let mut ranges = current.ranges.clone();
                    ranges[rule.tested_index] = accepted_range;
                    queue.push_back(PartRange {
                        ranges,
                        workflow: id.clone(),
                    })
                }
            }

            current.update_range(rule.tested_index, remaining_range);
            // if yes
            //      -> add condition to part range and handle the Destination
            //      -> also negate the condition and continue with the next rule
        }

        match default {
            Destination::Accept => accepted_ranges.push(current.ranges.clone()),
            Destination::Reject => (),
            Destination::Workflow(id) => queue.push_back(PartRange {
                ranges: current.ranges.clone(),
                workflow: id.clone(),
            }),
        }
    }

    println!("Accepted Ranges: {:?}", accepted_ranges);

    let result: usize = accepted_ranges
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|range| range.end - range.start)
                .product::<usize>()
        })
        .sum();

    // Ranges probably don't all overlap each other
    // Whenever i Accept a Range i have to save all it's settings and then probably just mark all those solutions as accepted
    // if i just combine all of them i end up with extra areas accepted due to not-existing overlap

    result.to_string()
}

/// Returns a tuple of options
///     first one matches the rule and the provided range
///     second is the part of the provided range, that's not contained in the range
fn find_matching_range(range: &Range<usize>, rule: &Rule) -> (Range<usize>, Range<usize>) {
    // range contains rule.value -> split range into 2 parts
    // value in range compared with rule.value gives the same ordering as rule -> entire range is contained
    // else range is not contained -> no return

    match rule.ordering {
        Ordering::Less => (range.start..rule.value, rule.value..range.end),
        Ordering::Greater => ((rule.value + 1)..range.end, range.start..(rule.value + 1)),
        _ => panic!("Should always be Less or Greater"),
    }
}

pub fn sum_up_part_range(ranges: &[Range<usize>]) -> u128 {
    let result = ranges
        .iter()
        .fold(1usize, |acc: usize, range: &Range<usize>| {
            acc * (range.end - range.start)
        });
    println!("{:?}", ranges);
    println!("adding {} to the total", result);
    result as u128
}

#[derive(Clone, Debug)]
struct PartRange {
    ranges: Vec<Range<usize>>,
    workflow: String,
}

impl PartRange {
    pub fn update_range(&mut self, index: usize, range: Range<usize>) {
        self.ranges[index] = range;
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, "167409079868000".to_string())
    }
}
