use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;
use regex_macro::{regex, Regex};
use aoc_shared::direction::Direction;
use aoc_shared::vector2d::Vector2D;

const REGEX_TEXT: &'static str = r"(\w+) (\d+) \(#([\d\w]{6})";

fn main() {
    // part_1();
    part_2();
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_18/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn part_1() {
    let path: Vec<DigInstruction> = read_file().lines()
        .map(|l| l.unwrap().parse::<DigInstruction>().unwrap())
        .collect();

    // println!("{:#?}", path);

    let edges_points = path.iter().map(|x| x.amount).sum::<usize>();

    let polygon = create_polygon(path.into_iter());
    let interior_points = get_interior_area(&polygon);

    let total_area = interior_points + (edges_points / 2) + 1;
    println!("{}", total_area);
}

fn part_2() {
    let path = read_file().lines()
        .map(|l| DigInstruction::from_hex_code(&l.unwrap()))
        .collect::<Vec<_>>();

    // println!("{:#?}", path);

    let edges_points = path.iter().map(|x| x.amount).sum::<usize>();

    let polygon = create_polygon(path.into_iter());
    let interior_points = get_interior_area(&polygon);

    let total_area = interior_points + (edges_points / 2) + 1;
    println!("{}", total_area);
}

fn create_polygon(path: impl Iterator<Item = DigInstruction>) -> Vec<Vector2D> {
    let mut polygon = vec![];
    let mut cur_pos = Vector2D::ZERO;
    for instruction in path {
        polygon.push(cur_pos);
        cur_pos = cur_pos + instruction.dig_vector();
    }
    polygon
}

fn get_interior_area(points: &[Vector2D]) -> usize {
    let n = points.len();
    let mut area = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].0 * points[j].1;
        area -= points[j].0 * points[i].1;
        // println!("{:?} <-> {:?}", points[i], points[j]);
    }

    area.abs() as usize / 2
}

#[derive(Debug, Clone)]
struct DigInstruction {
    direction: Direction,
    amount: usize,
}

impl DigInstruction {
    fn from_hex_code(s: &str) -> Self {
        let regex = regex!(REGEX_TEXT);
        let captures = regex.captures(s).ok_or("Regex failed").unwrap();
        let hex = captures.get(3).unwrap().as_str();
        let direction = match &hex[5..] {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => panic!("Unexpected case: {}", &hex[5..]),
        };
        let amount = usize::from_str_radix(&hex[..5], 16).unwrap();
        Self {
            direction,
            amount,
        }
    }

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
        let regex = regex!(REGEX_TEXT);
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

#[cfg(test)]
mod tests {
    use aoc_shared::vector2d::Vector2D;
    use crate::get_interior_area;

    #[test]
    fn get_simple_area() {
        let polygon = &[Vector2D(0, 0), Vector2D(2, 0), Vector2D(2, 1), Vector2D(0, 1)];

        let area = get_interior_area(polygon);

        assert_eq!(area, 2);
    }
}
