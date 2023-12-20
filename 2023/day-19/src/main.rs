use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-19.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    todo!()
}

fn part_two(input: &str) -> usize {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for MachinePart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<usize> = s
            .trim_matches('{')
            .trim_matches('}')
            .split(',')
            .map(|assignment| {
                let (attr, value) = assignment
                    .split_once('=')
                    .ok_or(anyhow!("Invalid syntax in MachinePart: {}", s))?;

                value.parse::<usize>().map_err(|_| {
                    anyhow!(
                        "Invalid syntax in MachinePart attribute: {}={}",
                        attr,
                        value
                    )
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Attribute {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Attribute {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err(anyhow!("Attribute {} does not exist", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    GreaterThan {
        attr: Attribute,
        value: usize,
        result: WorkflowResult,
    },
    LessThan {
        attr: Attribute,
        value: usize,
        result: WorkflowResult,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkflowResult {
    Accepted,
    Rejected,
    Jump(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, body) = s
            .split_once("{")
            .ok_or(anyhow!("Invalid syntax in Workflow: {}", s))?;

        let rules = body
            .trim_matches('}')
            .split(',')
            .map(|rule| {
                let (expr, result) = rule
                    .split_once(":")
                    .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                let attr = expr
                    .chars()
                    .nth(0)
                    .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;
                let cmp = expr
                    .chars()
                    .nth(1)
                    .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                let (_, value) = expr
                    .split_once(cmp)
                    .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                let result = match result {
                    "A" => WorkflowResult::Accepted,
                    "R" => WorkflowResult::Rejected,
                    workflow => WorkflowResult::Jump(workflow.to_owned()),
                };

                match cmp {
                    '<' => Ok(Rule::LessThan {
                        attr: attr.try_into()?,
                        value: value.parse()?,
                        result,
                    }),
                    '>' => Ok(Rule::GreaterThan {
                        attr: attr.try_into()?,
                        value: value.parse()?,
                        result,
                    }),
                    _ => Err(anyhow!("Invalid syntax in Rule: {}", rule)),
                }
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            label: label.to_owned(),
            rules,
        })
    }
}

impl Workflow {
    fn check(&self, part: &MachinePart) -> WorkflowResult {
        for rule in &self.rules {
            let (matches, result) = match rule {
                Rule::GreaterThan {
                    attr,
                    value,
                    result,
                } => (
                    match attr {
                        Attribute::X => part.x > *value,
                        Attribute::M => part.m > *value,
                        Attribute::A => part.a > *value,
                        Attribute::S => part.s > *value,
                    },
                    result,
                ),
                Rule::LessThan {
                    attr,
                    value,
                    result,
                } => (
                    match attr {
                        Attribute::X => part.x < *value,
                        Attribute::M => part.m < *value,
                        Attribute::A => part.a < *value,
                        Attribute::S => part.s < *value,
                    },
                    result,
                ),
            };

            if matches {
                continue;
            }

            return result.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;
    use rstest::*;

    #[rstest]
    #[case("{x=787,m=2655,a=1222,s=2876}", MachinePart { x: 787, m: 2655, a: 1222, s: 2876 })]
    #[case("{x=1679,m=44,a=2067,s=496}", MachinePart {x: 1679, m: 44, a: 2067, s: 496})]
    #[case("{x=2036,m=264,a=79,s=2244}", MachinePart {x: 2036, m: 264, a: 79, s: 2244})]
    #[case("{x=2461,m=1339,a=466,s=291}", MachinePart {x: 2461, m: 1339, a: 466, s: 291})]
    #[case("{x=2127,m=1623,a=2188,s=1013}", MachinePart {x: 2127, m: 1623, a: 2188, s: 1013})]
    fn test_parse_machine_part(
        #[case] s: &str,
        #[case] expected: MachinePart,
    ) -> anyhow::Result<()> {
        assert_eq!(s.parse::<MachinePart>()?, expected);
        Ok(())
    }

    #[test]
    fn test_part_one() {
        let input = indoc! {"

        "};

        assert_eq!(1, part_one(input));
    }
}
