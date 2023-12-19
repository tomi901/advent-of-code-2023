mod part;
mod workflow_map;
mod rule;
mod xmas_range;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex_macro::regex;
use crate::part::Part;
use crate::rule::Rule;
use crate::workflow_map::WorkflowMap;

fn main() {
    part_1();
}

fn part_1() {
    let reader = read_file();
    let mut lines = reader.lines();
    let workflow = WorkflowMap::from_lines(&mut lines);

    let mut sum = 0;
    for line_result in lines {
        let part = line_result.unwrap().parse::<Part>().unwrap();
        // print!("{:?} = ", part);
        if workflow.check_accepted(&part) {
            // println!("Accepted");
            sum += part.values_sum();
        } else {
            // println!("Rejected");
        }
    }
    println!("Sum: {sum}");
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_19/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

pub const STARTING_POINT: &str = "in";
pub const PROPERTIES_COUNT: usize = 4;

pub type XMAS = [usize; PROPERTIES_COUNT];

#[derive(Debug, Copy, Clone)]
pub enum Property {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl FromStr for Property {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Property::X),
            "m" => Ok(Property::M),
            "a" => Ok(Property::A),
            "s" => Ok(Property::S),
            _ => panic!("Not a property name: {}", s)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Destination {
    Accept,
    Reject,
    SendTo(String),
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            &_ => Self::SendTo(value.to_owned()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Condition(pub Property, pub Ordering, pub usize);

impl Condition {
    pub fn inverted(&self) -> Self {
        match self.1 {
            Ordering::Less => Condition(self.0, Ordering::Greater, self.2 - 1),
            Ordering::Greater => Condition(self.0, Ordering::Less, self.2 + 1),
            Ordering::Equal => panic!("Cannot invert equals."),
        }
    }
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let condition_regex = regex!(r"(\w+)([<>])(\d+)");
        let captures = condition_regex.captures(s).unwrap();
        let property = captures.get(1).unwrap().as_str().parse::<Property>()?;
        let operator = match captures.get(2).unwrap().as_str() {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            &_ => panic!("Unexpected condition operator")
        };
        let value = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Ok(Self(property, operator, value))
    }
}

#[derive(Debug)]
pub struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn get_destination_for_part(&self, part: &Part) -> Option<&Destination> {
        self.rules.iter().flat_map(|r| r.get_destination_for_part(&part)).next()
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let workflow_regex = regex!(r"(\w+)\{(.*)\}");
        let captures = workflow_regex.captures(s).unwrap();
        let id = captures.get(1).unwrap().as_str().to_owned();

        let payload = captures.get(2).unwrap().as_str();
        let rules = payload.split(',').map(Rule::from_str).collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            id,
            rules,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::{Condition, Destination, Property, Rule, Part};

    #[test]
    fn check_condition_pass() {
        let part = Part::new(787, 2655, 1222, 2876);
        let rule = Rule::with_condition(Condition(Property::A, Ordering::Less, 2006), Destination::SendTo("qkq".to_owned()));

        let result = rule.get_destination_for_part(&part);

        assert!(result.is_some());
        assert_eq!(result, Some(&Destination::SendTo("qkq".to_owned())));
    }

    #[test]
    fn check_condition_fail() {
        let part = Part::new(787000, 2655, 1222, 2876);
        let rule = Rule::with_condition(Condition(Property::A, Ordering::Less, 2006), Destination::SendTo("qkq".to_owned()));

        let result = rule.get_destination_for_part(&part);

        assert!(result.is_none());
    }
}
