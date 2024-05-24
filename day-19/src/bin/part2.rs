use std::{
    cmp::Ordering,
    collections::{self, VecDeque},
    ops::Range,
};

use aoc_util::range_util::combine_ranges;
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
    let mut accepted_ranges: Vec<PartRange> = vec![];

    let mut queue: VecDeque<PartRange> = VecDeque::new();
    queue.push_back(PartRange {
        ranges: vec![None; 4],
        workflow: "in".to_string(),
    });

    while let Some(queue_range) = queue.pop_front() {
        let mut current = queue_range.clone();
        // process current range and queue any potential children
        let (rules, default) = rules.get(&current.workflow).unwrap();
        for rule in rules {
            // check if part can match rule or no
            // if not -> ignore and move on
            let affected_range = &current.ranges[rule.tested_index];
            let (accepted_range, remaining_range) = find_matching_range(affected_range, rule);

            if let Some(accepted_range) = accepted_range {
                match &rule.destination {
                    Destination::Accept => {
                        current.update_range(rule.tested_index, accepted_range);
                        accepted_ranges.push(current.clone());
                    }
                    Destination::Reject => (),
                    Destination::Workflow(id) => {
                        let mut ranges = current.ranges.clone();
                        ranges[rule.tested_index] = Some(accepted_range);
                        queue.push_back(PartRange {
                            ranges,
                            workflow: id.clone(),
                        })
                    }
                }
            }

            if let Some(remaining_range) = remaining_range {
                current.update_range(rule.tested_index, remaining_range);
            }
            // if yes
            //      -> add condition to part range and handle the Destination
            //      -> also negate the condition and continue with the next rule
        }

        match default {
            Destination::Accept => accepted_ranges.push(current.clone()),
            Destination::Reject => (),
            Destination::Workflow(id) => queue.push_back(PartRange {
                ranges: current.ranges.clone(),
                workflow: id.clone(),
            }),
        }
    }

    let mut uncombined_ranges: Vec<Vec<Range<usize>>> = vec![vec![]; 4];
    for range in accepted_ranges {
        for (i, range) in range.ranges.into_iter().enumerate() {
            if let Some(r) = range {
                uncombined_ranges[i].push(r.clone());
            }
        }
    }

    println!("Uncombined Ranges: {:?}", uncombined_ranges);

    let combined_ranges: Vec<Vec<Range<usize>>> =
        uncombined_ranges.iter().map(combine_ranges).collect();

    println!("Combined Ranges: {:?}", combined_ranges);

    let result: usize = combined_ranges
        .iter()
        .map(|ranges| {
            ranges
                .iter()
                .fold(0, |acc, range| acc + (range.end - range.start))
        })
        .product();

    // Ranges probably don't all overlap each other
    // Whenever i Accept a Range i have to save all it's settings and then probably just mark all those solutions as accepted
    // if i just combine all of them i end up with extra areas accepted due to not-existing overlap

    result.to_string()
}

/// Returns a tuple of options
///     first one matches the rule and the provided range
///     second is the part of the provided range, that's not contained in the range
fn find_matching_range(
    range: &Option<Range<usize>>,
    rule: &Rule,
) -> (Option<Range<usize>>, Option<Range<usize>>) {
    // range contains rule.value -> split range into 2 parts
    // value in range compared with rule.value gives the same ordering as rule -> entire range is contained
    // else range is not contained -> no return

    match range {
        None => {
            // create new range based on Rule
            match rule.ordering {
                Ordering::Less => (Some(LOWER_BOUND..rule.value), None),
                Ordering::Greater => (Some((rule.value + 1)..UPPER_BOUND), None),
                _ => panic!("Should always be Less or Greater"),
            }
        }
        Some(range) if range.contains(&rule.value) => {
            // split range into 2 parts
            match rule.ordering {
                Ordering::Less => (Some(range.start..rule.value), Some(rule.value..range.end)),
                Ordering::Greater => (
                    Some((rule.value + 1)..range.end),
                    Some(range.start..(rule.value + 1)),
                ),
                _ => panic!("Should always be Less or Greater"),
            }
        }
        Some(range) if range.start.cmp(&rule.value) == rule.ordering => {
            // range is contained entirely
            (Some(range.clone()), None)
        }
        Some(range) => {
            // range is not contained at all
            (None, Some(range.clone()))
        }
    }
}

pub fn sum_up_part_range(ranges: &[Option<Range<usize>>]) -> u128 {
    let result = ranges.iter().fold(
        1usize,
        |acc: usize, range: &Option<Range<usize>>| match range {
            None => acc * (UPPER_BOUND - LOWER_BOUND),
            Some(r) => acc * (r.end - r.start),
        },
    );
    println!("{:?}", ranges);
    println!("adding {} to the total", result);
    result as u128
}

#[derive(Clone, Debug)]
struct PartRange {
    ranges: Vec<Option<Range<usize>>>,
    workflow: String,
}

impl PartRange {
    pub fn update_range(&mut self, index: usize, range: Range<usize>) {
        self.ranges[index] = Some(range);
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

    #[test]
    fn test_basic_sum() {
        let result = process(
            "in{a<2:A,x>2:R,m>2:R,s<3:xp,R}
xp{s<2:A,R}",
        );
        assert_eq!(result, 2.to_string())
    }
}
