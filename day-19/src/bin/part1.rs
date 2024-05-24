use day_19::{process_rules, Destination, Rule};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut iter = input.lines();
    let rule_string: Vec<&str> = iter.by_ref().take_while(|line| !line.is_empty()).collect();
    let _ = iter.by_ref().skip(1);
    let parts_string: Vec<&str> = iter.collect();

    let rules = process_rules(rule_string);
    let parts: Vec<Vec<usize>> = parts_string.into_iter().map(parse_part).collect();

    let result: usize = parts
        .iter()
        .filter_map(|part| {
            let mut workflow = rules.get("in").unwrap();
            let mut destination = handle_workflow(workflow, part);
            while let Destination::Workflow(id) = destination.clone() {
                workflow = rules.get(&id).unwrap();
                destination = handle_workflow(workflow, part);
            }

            match destination {
                Destination::Accept => Some(part.iter().sum::<usize>()),
                _ => None,
            }
        })
        .sum();

    result.to_string()
}

pub fn handle_workflow(workflow: &(Vec<Rule>, Destination), part: &[usize]) -> Destination {
    for rule in &workflow.0 {
        if let Some(dest) = rule.process_rule(part) {
            return dest.clone();
        }
    }
    workflow.1.clone()
}

pub fn parse_part(value_string: &str) -> Vec<usize> {
    let s = value_string.trim_matches(|c| c == '{' || c == '}');
    s.split(',')
        .map(|value| value[2..].parse().unwrap())
        .collect()
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
        assert_eq!(result, "19114".to_string())
    }
}
