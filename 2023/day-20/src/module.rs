use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Module {
    Button,
    Repeater {
        label: String,
        outputs: Vec<String>,
    },
    Broadcaster {
        outputs: Vec<String>,
    },
    FlipFlop {
        label: String,
        last_pulse: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        label: String,
        memory: HashMap<String, bool>,
        outputs: Vec<String>,
    },
}

impl Module {
    pub fn label(&self) -> &str {
        match self {
            Module::Button => "button",
            Module::Repeater { label, .. } => &label,
            Module::Broadcaster { .. } => "broadcaster",
            Module::FlipFlop { label, .. } => &label,
            Module::Conjunction { label, .. } => &label,
        }
    }

    pub fn outputs(&self) -> Vec<String> {
        match self {
            Module::Button => vec!["broadcaster".to_string()],
            Module::Repeater { outputs, .. } => outputs.clone(),
            Module::Broadcaster { outputs } => outputs.clone(),
            Module::FlipFlop { outputs, .. } => outputs.clone(),
            Module::Conjunction { outputs, .. } => outputs.clone(),
        }
    }

    pub fn process_pulse(&mut self, sender: String, pulse: bool) -> Option<bool> {
        match self {
            Module::Button => Some(pulse),
            Module::Repeater { .. } => Some(pulse),
            Module::Broadcaster { .. } => Some(pulse),
            Module::FlipFlop { last_pulse, .. } => {
                if !pulse {
                    *last_pulse = !(*last_pulse);
                    Some(*last_pulse)
                } else {
                    None
                }
            }
            Module::Conjunction { memory, .. } => {
                memory.insert(sender, pulse);

                if memory.values().all(|&e| e == true) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
        }
    }

    pub fn update_inputs(&mut self, inputs: Vec<String>) {
        match self {
            Module::Conjunction { memory, .. } => {
                for input in inputs {
                    memory.insert(input, false);
                }
            }
            _ => (),
        }
    }
}
