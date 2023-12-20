use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::action_queue::{Action, ActionQueue};
use crate::Pulse;

const FLIP_FLOP_PREFIX: char = '%';
const CONJUNCTION_PREFIX: char = '&';

#[derive(Debug)]
pub struct Module {
    inputs: HashSet<String>,
    id: String,
    info: ModuleInfo,
    outputs: Vec<String>,
}

impl Module {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn add_input(&mut self, input_id: &str) -> bool {
        self.inputs.insert(input_id.to_owned())
    }

    pub fn outputs(&self) -> &[String] {
        &self.outputs[..]
    }

    pub fn process_pulse(&mut self, from: &str, pulse: Pulse, action_queue: &mut ActionQueue) {
        let output_pulse = match &mut self.info {
            ModuleInfo::Default => Some(pulse),
            ModuleInfo::FlipFlop(ref mut state) if pulse == Pulse::Low => {
                *state = state.invert();
                Some(*state)
            }
            ModuleInfo::FlipFlop(_) => None,
            ModuleInfo::Conjunction(ref mut state) => {
                match pulse {
                    Pulse::Low => { state.remove(from); }
                    Pulse::High => { state.insert(from.to_owned()); }
                }
                Some(Pulse::from(state.len() < self.inputs.len()))
            },
        };
        if let Some(output) = output_pulse {
            for target in self.outputs.iter() {
                let action = Action::new(&self.id, output, target);
                action_queue.push(action);
            }
        }
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("->");
        let from_str = split.next().unwrap().trim();
        let (id, info) = if from_str.starts_with(FLIP_FLOP_PREFIX) {
            (from_str.trim_start_matches(FLIP_FLOP_PREFIX), ModuleInfo::FlipFlop(Pulse::Low))
        } else if from_str.starts_with(CONJUNCTION_PREFIX) {
            (from_str.trim_start_matches(CONJUNCTION_PREFIX), ModuleInfo::Conjunction(HashSet::default()))
        } else {
            (from_str, ModuleInfo::Default)
        };

        let targets = split
            .next()
            .unwrap()
            .split(',')
            .map(|t| t.trim().to_owned())
            .collect::<Vec<_>>();

        Ok(Self {
            inputs: HashSet::default(),
            id: id.to_owned(),
            info,
            outputs: targets,
        })
    }
}

#[derive(Debug)]
pub enum ModuleInfo {
    Default,
    FlipFlop(Pulse),
    Conjunction(HashSet<String>),
}
