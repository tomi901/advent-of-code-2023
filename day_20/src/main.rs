use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::modules_network::ModulesNetwork;

mod module;
mod action_queue;
mod modules_network;

fn main() {
    part_1();
}

fn part_1() {
    let mut network = ModulesNetwork::from_reader(read_file());
    // println!("{:#?}", network);
    let result = (0..1000).map(|_| network.start_process()).reduce(|acc, e| acc + e).unwrap();
    println!("{result:?}");
    println!("{}", result.product());
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_20/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pulse {
    Low,
    High,
}

impl Pulse {
    pub fn invert(&self) -> Pulse {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        if value {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => write!(f, "low"),
            Pulse::High => write!(f, "high"),
        }
    }
}
