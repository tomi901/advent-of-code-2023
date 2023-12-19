use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::{Destination, Workflow};
use crate::part::Part;
use crate::rule::Rule;
use crate::xmas_range::XMASRange;

pub struct WorkflowMap(HashMap<String, Workflow>);

impl WorkflowMap {
    pub fn from_lines<B: BufRead>(lines: &mut Lines<B>) -> Self {
        let mut map = HashMap::default();
        for line_result in lines {
            let line = line_result.unwrap();
            if line.is_empty() {
                break;
            }
            let workflow = Workflow::from_str(&line).unwrap();
            map.insert(workflow.id.to_owned(), workflow);
        }
        Self(map)
    }

    pub fn check_accepted(&self, part: &Part) -> bool {
        let mut cur_workflow = self.starting_workflow();
        loop {
            // println!("{}", cur_workflow.id);
            let next_destination = cur_workflow.get_destination_for_part(&part)
                .expect("No destination found!");
            match next_destination {
                Destination::Accept => return true,
                Destination::Reject => return false,
                Destination::SendTo(id) => {
                    cur_workflow = self.0.get(id).expect("No next workflow found!");
                }
            }
        }
    }

    pub fn starting_workflow(&self) -> &Workflow {
        self.0.get("in").expect("No \"in\" workflow.")
    }

    pub fn calculate_combinations(&self, range: RangeInclusive<usize>) -> usize {
        let ranges = XMASRange::from_range(range);
        self.calculate_combinations_cached(ranges, self.starting_workflow())
    }

    pub fn calculate_combinations_cached(
        &self,
        ranges: XMASRange,
        from_workflow: &Workflow,
    ) -> usize {
        let mut cur_ranges = ranges.clone();
        let mut accepted = 0;
        for rule in from_workflow.rules.iter() {
            if let Some(condition) = rule.condition() {
                // TODO
                continue;
            }
        }
        panic!("Reached end of loop!")
    }
}
