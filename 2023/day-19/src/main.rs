use crate::workflow::{Rule, Workflow, WorkflowResult};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::{env, fs, io::Read};

mod workflow;

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-19.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| line.parse::<Workflow>().unwrap())
        .map(|workflow| (workflow.label.clone(), workflow))
        .collect();

    let parts: Vec<[usize; 4]> = parts
        .lines()
        .map(|line| {
            let v = line
                .trim_matches('{')
                .trim_matches('}')
                .split(',')
                .map(|assignment| {
                    let (attr, value) = assignment
                        .split_once('=')
                        .ok_or(anyhow!("Invalid syntax in MachinePart: {}", line))
                        .unwrap();

                    value
                        .parse::<usize>()
                        .map_err(|_| {
                            anyhow!(
                                "Invalid syntax in MachinePart attribute: {}={}",
                                attr,
                                value
                            )
                        })
                        .unwrap()
                })
                .collect_vec();

            [v[0], v[1], v[2], v[3]]
        })
        .collect_vec();

    parts
        .iter()
        .filter_map(|part| {
            let mut current_workflow = workflows.get("in").unwrap();

            while let WorkflowResult::Jump(next) = current_workflow.check(&part) {
                // This could result in an infinite loop...
                current_workflow = workflows.get(&next).unwrap();
            }

            match current_workflow.check(&part) {
                WorkflowResult::Accepted => Some(part),
                WorkflowResult::Rejected => None,
                WorkflowResult::Jump(_) => panic!(),
            }
        })
        .map(|part| part.iter().sum::<usize>())
        .sum()
}

fn count_matches(
    workflows: &HashMap<String, Workflow>,
    current_workflow: String,
    rule_index: usize,
    value_sets: [HashSet<usize>; 4],
) -> usize {
    if let Some(workflow) = workflows.get(&current_workflow) {
        if let Some(rule) = workflow.rules.get(rule_index) {
            let (non_matching_range, matching_range) = match rule {
                Rule::GreaterThan {
                    attr: _,
                    value,
                    result: _,
                } => (1..=*value, *value + 1..=4000),
                Rule::LessThan {
                    attr: _,
                    value,
                    result: _,
                } => (*value..=4000, 1..=*value + 1),
                _ => (1..=0, 1..=4000),
            };

            let matches = match rule.result() {
                // Base case
                WorkflowResult::Rejected => 0,
                WorkflowResult::Accepted => value_sets.iter().map(|set| set.len()).product(),

                // Recursive case
                WorkflowResult::Jump(next_workflow) => {
                    let mut new_sets = value_sets.clone();
                    if let Some(attr) = rule.attr() {
                        new_sets[attr] = new_sets[attr]
                            .difference(&HashSet::from_iter(non_matching_range))
                            .map(|e| *e)
                            .collect();
                    }
                    count_matches(workflows, next_workflow, 0, new_sets)
                }
            };

            let mut new_sets = value_sets.clone();
            if let Some(attr) = rule.attr() {
                new_sets[attr] = new_sets[attr]
                    .difference(&HashSet::from_iter(matching_range))
                    .map(|e| *e)
                    .collect();
            }
            let non_matches = count_matches(workflows, current_workflow, rule_index + 1, new_sets);

            return matches + non_matches;
        }
    }
    0
}

fn part_two(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| line.parse::<Workflow>().unwrap())
        .map(|workflow| (workflow.label.clone(), workflow))
        .collect();

    let value_sets = [
        HashSet::from_iter(1..=4000),
        HashSet::from_iter(1..=4000),
        HashSet::from_iter(1..=4000),
        HashSet::from_iter(1..=4000),
    ];

    count_matches(&workflows, "in".to_owned(), 0, value_sets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
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
            {x=2127,m=1623,a=2188,s=1013}
        "};

        assert_eq!(19114, part_one(input));
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
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
            {x=2127,m=1623,a=2188,s=1013}
        "};

        assert_eq!(167409079868000, part_two(input));
    }
}
