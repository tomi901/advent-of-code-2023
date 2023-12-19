use std::str::FromStr;
use crate::{Condition, Destination};
use crate::part::Part;

#[derive(Debug, Clone)]
pub struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

impl Rule {
    pub fn condition(&self) -> Option<&Condition> {
        self.condition.as_ref()
    }

    pub fn destination(&self) -> &Destination {
        &self.destination
    }

    pub fn with_condition(condition: Condition, destination: Destination) -> Self {
        Self {
            condition: Some(condition),
            destination,
        }
    }

    pub fn no_condition(destination: Destination) -> Self {
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