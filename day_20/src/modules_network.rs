use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Add;
use crate::action_queue::{Action, ActionQueue};
use crate::module::Module;
use crate::Pulse;

#[derive(Debug)]
pub struct ModulesNetwork {
    modules: HashMap<String, Module>,
}

impl ModulesNetwork {
    pub fn from_reader(reader: impl BufRead) -> Self {
        let mut modules = HashMap::default();
        let mut io = vec![];
        for line_result in reader.lines() {
            let new_module = line_result.unwrap().parse::<Module>().unwrap();
            io.extend(new_module.outputs().iter().map(|s| (new_module.id().to_owned(), s.to_owned())));
            modules.insert(new_module.id().to_owned(), new_module);
        }
        for (input, output) in io {
            modules.get_mut(&output).and_then(|o| Some(o.add_input(&input)));
        }
        Self { modules }
    }

    pub fn start_process(&mut self) -> OutputResult {
        let mut queue = ActionQueue::new();
        queue.push(Action::new("button", Pulse::Low, "broadcaster"));

        let mut output = OutputResult::default();
        let mut i = 0;
        while i < queue.len() {
            let action = &queue[i];
            i += 1;

            let pulse = action.pulse;
            match pulse {
                Pulse::Low => { output.low_count += 1; }
                Pulse::High => { output.high_count += 1; }
            }

            if let Some(target) = self.modules.get_mut(&action.to) {
                target.process_pulse(&action.from.to_owned(), pulse, &mut queue);
            }
        }
        // queue.iter().for_each(|a| println!("{}", a));
        output
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct OutputResult {
    pub low_count: usize,
    pub high_count: usize,
}

impl OutputResult {
    pub fn product(&self) -> usize {
        self.low_count * self.high_count
    }
}

impl Add for OutputResult {
    type Output = OutputResult;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            low_count: self.low_count + rhs.low_count,
            high_count: self.high_count + rhs.high_count,
        }
    }
}
