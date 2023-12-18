use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;
use regex_macro::{regex, Regex};
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::Direction;
use aoc_shared::vector2d::Vector2D;

fn main() {
    let path = std::env::current_dir().unwrap().join("day_18/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let instructions: Vec<DigInstruction> = reader.lines()
        .map(|l| l.unwrap().parse::<DigInstruction>().unwrap())
        .collect();
    
    let bounds = get_bounds(instructions.iter().map(DigInstruction::dig_vector));
    println!("Bounds {:?} -> {:?}", bounds.0, bounds.1);
    
    let offset =
}

fn create_map(path: &Vec<DigInstruction>)

fn get_bounds(movements: impl Iterator<Item = Vector2D>) -> (Vector2D, Vector2D) {
    let mut cur_point = Vector2D::ZERO;
    let mut min = Vector2D::ZERO;
    let mut max = Vector2D::ZERO;
    for movement in movements {
        cur_point = cur_point.add(movement);
        min = min.min_2d(cur_point);
        max = max.max_2d(cur_point);
    }
    (min, max + Vector2D(1, 1))
}

#[derive(Debug, Clone)]
struct DigInstruction {
    direction: Direction,
    amount: usize,
}

impl DigInstruction {
    fn parse_direction(s: &str) -> Direction {
        match s {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => panic!("Invalid direction {}", s),
        }
    }
    
    fn dig_vector(&self) -> Vector2D {
        Vector2D::from(self.direction) * self.amount.clone()
    }
}

impl FromStr for DigInstruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex!(r"(\w+) (\d+) \(#([\d\w]{6})");
        let captures = regex.captures(s).ok_or("Regex failed")?;
        let direction = DigInstruction::parse_direction(
            captures.get(1).ok_or("No direction in match")?.as_str(),
        );
        let amount = captures.get(2).ok_or("No direction in match")?.as_str()
            .parse::<usize>().unwrap();
        Ok(Self {
            direction,
            amount,
        })
    }
}
