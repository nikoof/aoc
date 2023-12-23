use crate::module::*;
use anyhow::Result;
use num::integer::lcm;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::{env, fs, io::Read};

mod module;

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-19.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn parse_modules(input: &str) -> HashMap<&str, Rc<RefCell<Module>>> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (label, outputs) = line.split_once(" -> ").unwrap();

            let (t, label) = label.split_at(1);
            let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

            let (label, module): (&str, Rc<RefCell<Module>>) = match t {
                "%" => (
                    label,
                    Rc::new(RefCell::new(Module::FlipFlop {
                        label: label.to_string(),
                        last_pulse: false,
                        outputs,
                    })),
                ),
                "&" => (
                    label,
                    Rc::new(RefCell::new(Module::Conjunction {
                        label: label.to_string(),
                        memory: HashMap::new(),
                        outputs,
                    })),
                ),
                "b" => (
                    "broadcaster",
                    Rc::new(RefCell::new(Module::Broadcaster { outputs })),
                ),
                _ => (
                    label,
                    Rc::new(RefCell::new(Module::Repeater {
                        label: label.to_string(),
                        outputs,
                    })),
                ),
            };

            (label, module)
        })
        .collect::<HashMap<&str, Rc<RefCell<Module>>>>();
    modules.insert("button", Rc::new(RefCell::new(Module::Button)));

    modules
        .values()
        .filter(|module| match *module.borrow() {
            Module::Conjunction { .. } => true,
            _ => false,
        })
        .for_each(|conjunction| {
            let inputs = modules
                .iter()
                .filter(|(_, module)| {
                    module
                        .borrow()
                        .outputs()
                        .iter()
                        .find(|output| output == &conjunction.borrow().label())
                        .is_some()
                })
                .map(|(label, _)| label.to_string())
                .collect();

            conjunction.borrow_mut().update_inputs(inputs);
        });

    modules
}

fn part_one(input: &str) -> usize {
    let modules = parse_modules(input);

    let (mut low, mut high) = (0, 0);
    let mut queue = VecDeque::new();
    for _ in 0..1000 {
        queue.push_back(("button".to_string(), false));

        while let Some((sender, pulse)) = queue.pop_front() {
            let sender_module = modules.get(sender.as_str()).unwrap().borrow();
            for output in sender_module.outputs() {
                match pulse {
                    true => high += 1,
                    false => low += 1,
                }

                if let Some(output_module) = modules.get(output.as_str()) {
                    if let Some(output_pulse) = output_module
                        .borrow_mut()
                        .process_pulse(sender.to_string(), pulse)
                    {
                        queue.push_back((output.clone(), output_pulse))
                    }
                }
            }
        }
    }

    low * high
}

/* By analyzing the input, we can observe that for `rx` to receive a low pulse, `&hf` must send a
* low pulse. For that to happen, all inputs of `&hf` need to send a high pulse. Therefore, we
* calculate how many steps it takes for the inputs of `&hf` to send a high pulse and then return
* the LCM of those. */
fn part_two(input: &str) -> usize {
    let modules = parse_modules(input);

    let mut i: usize = 0;
    let mut cycle_lengths = HashMap::new();
    let mut queue = VecDeque::new();

    'outer: loop {
        queue.push_back(("button".to_string(), false));
        i += 1;

        while let Some((sender, pulse)) = queue.pop_front() {
            for output in ["nd", "pc", "vd", "tx"] {
                if cycle_lengths.get(output).is_none() && output == &sender && pulse == true {
                    cycle_lengths.insert(output, i);
                }

                if cycle_lengths.len() == 4 {
                    break 'outer;
                }
            }

            let sender_module = modules.get(sender.as_str()).unwrap().borrow();
            for output in sender_module.outputs() {
                if let Some(output_module) = modules.get(output.as_str()) {
                    if let Some(output_pulse) = output_module
                        .borrow_mut()
                        .process_pulse(sender.to_string(), pulse)
                    {
                        queue.push_back((output.clone(), output_pulse))
                    }
                }
            }
        }
    }

    println!("{:#?}", cycle_lengths);
    cycle_lengths.values().cloned().fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;
    use rstest::*;

    #[rstest]
    #[case(
        indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "},
        32000000
    )]
    #[case(
        indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "},
        11687500
    )]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_one(input), expected);
    }
}
