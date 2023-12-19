use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::{Property, XMAS};

#[derive(Default)]
pub struct Part(XMAS);

impl Debug for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl Part {
    pub fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self([x, m, a, s])
    }

    pub fn get(&self, property: Property) -> usize {
        self.0[property as usize]
    }

    pub fn set(&mut self, property: Property, value: usize) {
        self.0[property as usize] = value;
    }

    pub fn values_sum(&self) -> usize {
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
