use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, write};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;
use regex_macro::regex;

const STARTING_POINT: &str = "in";

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

#[derive(Debug, Copy, Clone)]
enum Property {
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

type XMAS = [usize; 4];

#[derive(Default)]
struct Part(XMAS);

impl Debug for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self([x, m, a, s])
    }

    fn get(&self, property: Property) -> usize {
        self.0[property as usize]
    }

    fn set(&mut self, property: Property, value: usize) {
        self.0[property as usize] = value;
    }

    fn values_sum(&self) -> usize {
        self.0.iter().sum()
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim_start_matches('{').trim_end_matches('}');
        let mut part = Part::default();
        for prop_value in trimmed.split(',') {
            let mut prop_value_split = prop_value.split('=');
            let property = prop_value_split.next().unwrap().parse::<Property>()?;
            let value = prop_value_split.next().unwrap().parse::<usize>().unwrap();
            part.set(property, value);
        }
        Ok(part)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Destination {
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
struct Condition(Property, Ordering, usize);

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

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

impl Rule {
    fn with_condition(condition: Condition, destination: Destination) -> Self {
        Self {
            condition: Some(condition),
            destination,
        }
    }

    fn no_condition(destination: Destination) -> Self {
        Self {
            condition: None,
            destination,
        }
    }

    pub fn get_destination_for_part(&self, part: &Part) -> Option<&Destination> {
        self.check_condition_for_part(part).then_some(&self.destination)
    }

    pub fn check_condition_for_part(&self, part: &Part) -> bool {
        if self.condition.is_none() {
            // println!("Sending directly to: {:?}", self.destination);
            return true;
        }
        let Condition(property, operation, value) = self.condition.unwrap();
        let result = part.get(property).cmp(&value);
        // println!("{:?} {:?} {:?} = {:?}", property, operation, value, result);
        result == operation
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let mut split = s.split(':');
            let condition = split.next().unwrap().parse::<Condition>()?;
            let destination = Destination::from(split.next().unwrap());
            Ok(Self::with_condition(condition, destination))
        } else {
            Ok(Self::no_condition(Destination::from(s)))
        }
    }
}

#[derive(Debug)]
struct Workflow {
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

struct WorkflowMap(HashMap<String, Workflow>);

impl WorkflowMap {
    fn check_accepted(&self, part: &Part) -> bool {
        let mut cur_workflow = self.0.get("in").expect("No \"in\" workflow.");
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
}

impl WorkflowMap {
    fn from_lines<B: BufRead>(lines: &mut Lines<B>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::{Condition, Destination, Part, Property, Rule};

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
