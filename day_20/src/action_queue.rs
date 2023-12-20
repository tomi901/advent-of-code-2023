use std::fmt::{Display, Formatter};
use crate::Pulse;

pub type ActionQueue = Vec<Action>;

#[derive(Debug)]
pub struct Action {
    pub from: String,
    pub pulse: Pulse,
    pub to: String,
}

impl Action {
    pub fn new(from: &str, pulse: Pulse, to: &str) -> Self {
        Self { from: from.to_owned(), pulse, to: to.to_owned() }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.pulse, self.to)
    }
}
