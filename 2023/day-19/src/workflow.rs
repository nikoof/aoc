use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    GreaterThan {
        attr: usize,
        value: usize,
        result: WorkflowResult,
    },
    LessThan {
        attr: usize,
        value: usize,
        result: WorkflowResult,
    },
    Always {
        result: WorkflowResult,
    },
}

impl Rule {
    pub fn attr(&self) -> Option<usize> {
        match self {
            Rule::GreaterThan {
                attr,
                value: _,
                result: _,
            } => Some(*attr),
            Rule::LessThan {
                attr,
                value: _,
                result: _,
            } => Some(*attr),
            Rule::Always { .. } => None,
        }
    }
    pub fn result(&self) -> WorkflowResult {
        match self {
            Rule::GreaterThan {
                attr: _,
                value: _,
                result,
            } => result.clone(),
            Rule::LessThan {
                attr: _,
                value: _,
                result,
            } => result.clone(),
            Rule::Always { result } => result.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub label: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowResult {
    Accepted,
    Rejected,
    Jump(String),
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, body) = s
            .split_once("{")
            .ok_or(anyhow!("Invalid syntax in Workflow: {}", s))?;

        let rules: Vec<Rule> = body
            .trim_matches('}')
            .split(',')
            .map(|rule| {
                let (expr, result) = match rule.split_once(":") {
                    Some((expr, result)) => (Some(expr), result),
                    None => (None, rule),
                };

                let result = match result {
                    "A" => WorkflowResult::Accepted,
                    "R" => WorkflowResult::Rejected,
                    workflow => WorkflowResult::Jump(workflow.to_owned()),
                };

                if let Some(expr) = expr {
                    let attr = expr
                        .chars()
                        .nth(0)
                        .map(|ch| match ch {
                            'x' => Some(0),
                            'm' => Some(1),
                            'a' => Some(2),
                            's' => Some(3),
                            _ => None,
                        })
                        .flatten()
                        .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                    let cmp = expr
                        .chars()
                        .nth(1)
                        .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                    let (_, value) = expr
                        .split_once(cmp)
                        .ok_or(anyhow!("Invalid syntax in Rule: {}", rule))?;

                    match cmp {
                        '<' => Ok(Rule::LessThan {
                            attr,
                            value: value.parse()?,
                            result,
                        }),
                        '>' => Ok(Rule::GreaterThan {
                            attr,
                            value: value.parse()?,
                            result,
                        }),
                        _ => Err(anyhow!("Invalid syntax in Rule: {}", rule)),
                    }
                } else {
                    Ok(Rule::Always { result })
                }
            })
            .collect::<Result<_, Self::Err>>()?;

        if let Rule::Always { .. } = rules.last().unwrap() {
            Ok(Self {
                label: label.to_owned(),
                rules,
            })
        } else {
            Err(anyhow!("Last rule in Workflow {} should always match", s))
        }
    }
}

impl Workflow {
    pub fn check(&self, part: &[usize; 4]) -> WorkflowResult {
        for rule in &self.rules {
            let (matches, result) = match rule {
                Rule::GreaterThan {
                    attr,
                    value,
                    result,
                } => (part[*attr] > *value, result),
                Rule::LessThan {
                    attr,
                    value,
                    result,
                } => (part[*attr] < *value, result),
                Rule::Always { result } => (true, result),
            };

            if matches {
                return result.clone();
            }
        }

        if let Rule::Always { result } = self
            .rules
            .last()
            .expect("Valid Workflow last rule should always match")
        {
            return result.clone();
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("px{a<2006:qkq,m>2090:A,rfg}", Workflow {
        label: "px".to_owned(),
        rules: vec![
            Rule::LessThan { attr: 2, value: 2006, result: WorkflowResult::Jump("qkq".to_owned()) },
            Rule::GreaterThan { attr: 1, value: 2090, result: WorkflowResult::Accepted },
            Rule::Always { result: WorkflowResult::Jump("rfg".to_owned()) }
        ]
    })]
    fn test_parse_workflow(#[case] s: &str, #[case] expected: Workflow) -> anyhow::Result<()> {
        assert_eq!(s.parse::<Workflow>()?, expected);

        Ok(())
    }
}
