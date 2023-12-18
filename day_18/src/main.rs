use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;
use regex_macro::regex;
use aoc_shared::direction::Direction;
use aoc_shared::vector2d::Vector2D;

fn main() {
    part_1();
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_18/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn part_1() {
    let path: Vec<DigInstruction> = read_file().lines()
        .map(|l| l.unwrap().parse::<DigInstruction>().unwrap())
        .collect();

    let mut map = DigMap::with_starting_path(&path);
    println!("Map is {}x{}", map.width(), map.height());

    println!();
    // print!("{}", map);
    println!("{} dug.", map.dig_count());

    map.dig_interior();
    println!();
    // print!("{}", map);
    println!("{} dug.", map.dig_count());
}

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

struct DigMap {
    dug: HashMap<Vector2D, Option<Direction>>,
    from: Vector2D,
    to: Vector2D,
}

impl DigMap {
    fn with_starting_path(path: &Vec<DigInstruction>) -> Self {
        let bounds = get_bounds(path.iter().map(DigInstruction::dig_vector));
        println!("Bounds {:?} -> {:?}", bounds.0, bounds.1);

        let mut dug = HashMap::default();
        let mut cur_point = Vector2D::ZERO;
        for instruction in path {
            let direction = instruction.direction;
            let move_towards: Vector2D = direction.into();
            if direction == Direction::South {
                dug.insert(cur_point, Some(Direction::South));
            }
            for i in 0..instruction.amount {
                cur_point = cur_point + move_towards;
                if direction != Direction::South || i < instruction.amount - 1 {
                    dug.insert(cur_point, Some(direction));
                } else {
                    dug.insert(cur_point, None);
                }
            }
        }
        Self {
            dug,
            from: bounds.0,
            to: bounds.1,
        }
    }

    fn width(&self) -> usize {
        self.from.0.abs_diff(self.to.0)
    }

    fn height(&self) -> usize {
        self.from.1.abs_diff(self.to.1)
    }

    fn dig_count(&self) -> usize {
        self.dug.len()
    }

    fn dig(&mut self, point: &Vector2D) {
        self.dug.insert(point.clone(), None);
    }

    fn is_dug(&self, point: &Vector2D) -> bool {
        self.dug.contains_key(point)
    }

    fn dig_interior(&mut self) {
        for y in self.from.1..self.to.1 {
            let mut digging = false;
            for x in self.from.0..self.to.0 {
                let point = Vector2D(x, y);
                if let Some(d) = self.dug.get(&point).and_then(|&d| d) {
                    if d == Direction::South || d == Direction::North {
                        digging = !digging;
                    }
                }

                if digging && !self.is_dug(&point) {
                    self.dig(&point);
                }
            }
        }
    }

    fn get_display_char(&self, point: &Vector2D) -> char {
        let dig_point = self.dug.get(&point);
        if dig_point.is_none() {
            return '.';
        }

        match dig_point.unwrap() {
            Some(d) => match d {
                Direction::North => 'A',
                Direction::East => '>',
                Direction::South => 'V',
                Direction::West => '<'
            },
            None => '#',
        }
    }
}

impl Display for DigMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const MAX_SHOWN_COLUMNS: isize = 24;
        for y in self.from.1..self.to.1 {
            for x in self.from.0..min(self.to.0, MAX_SHOWN_COLUMNS) {
                let point = Vector2D(x, y);
                let ch = self.get_display_char(&point);
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
