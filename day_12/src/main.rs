use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let row = Row::from_str(&line).unwrap();
        println!("{:?}", &row);
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
    expected_sequence: Vec<usize>,
}

impl Row {
    fn find_possible_combinations(&self) -> usize {
        todo!()
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        
        let tiles_str = split.next().expect("No tiles found in str.");
        let tiles: Vec<_> = tiles_str
            .chars()
            .map(|c| Tile::try_from(c).unwrap())
            .collect();

        let sequence_str = split.next().expect("No sequence found in str.");
        let expected_sequence: Vec<_> = sequence_str
            .split(',')
            .map(|num| num.parse::<usize>())
            .collect::<Result<_, _>>()
            .unwrap();
        
        Ok(Row { tiles, expected_sequence })
    }
}

#[derive(Debug)]
enum Tile {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Debug)]
struct InvalidCharError {
    c: char,
}

impl TryFrom<char> for Tile {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(InvalidCharError { c: value }),
        }
    }
}
