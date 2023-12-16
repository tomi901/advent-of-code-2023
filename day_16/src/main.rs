use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = std::env::current_dir().unwrap().join("day_16/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    
    let map = Map::try_from(&mut reader).unwrap();
    println!("{}", map);
    
    let points = map.get_beam_positions(Beam::starting());
    map.display_visited(&points);
    
    println!("{}", points.len())
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Vector2D(isize, isize);

impl From<Direction> for Vector2D {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Vector2D(0, -1),
            Direction::East => Vector2D(1, 0),
            Direction::South => Vector2D(0, 1),
            Direction::West => Vector2D(-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Point2D(usize, usize);

impl Point2D {
    const ZERO: Self = Point2D(0, 0);
    
    fn try_move(&self, amount: Vector2D) -> Option<Self> {
        let result = checked_add(self.0, amount.0)
            .and_then(|x| checked_add(self.1, amount.1)
                .and_then(|y| Some(Point2D(x, y))));
        // println!("{:?} + {:?} = {:?}", self, amount, result);
        return result;
        
        fn checked_add(a: usize, b: isize) -> Option<usize> {
            if b >= 0 {
                Some(a + b as usize)
            } else {
                let b_abs = b.abs() as usize;
                a.checked_sub(b_abs)
            }
        }
    }
    
    fn try_move_one(&self, direction: Direction) -> Option<Self> {
        self.try_move(Vector2D::from(direction))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Beam(Point2D, Direction);

impl Beam {
    fn starting() -> Self {
        Self(Point2D::ZERO, Direction::East)
    }
    
    fn try_move_one(&self) -> Option<Self> {
        self.0.try_move_one(self.1)
            .map(|new_pos| Self(new_pos, self.1))
    }

    fn try_move_one_in(&self, map: &Map) -> Option<Self> {
        self.try_move_one()
            .filter(|b| map.is_point_inside(b.0))
    }

    fn try_move_one_towards(&self, direction: Direction) -> Option<Self> {
        let new_beam = Self(self.0, direction);
        new_beam.try_move_one()
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row_i in 0..self.height {
            let start_i = row_i * self.width;
            for tile in &self.tiles[start_i..(start_i + self.width)] {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn display_visited(&self, visited: &HashSet<Point2D>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point2D(x, y);
                if visited.contains(&point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    
    fn try_from(input: &mut impl BufRead) -> Option<Self> {
        let mut lines = input.lines().flatten().take_while(|l| !l.is_empty());
        let first_line = match lines.next() {
            Some(line) => line,
            None => return None,
        };
        let mut tiles: Vec<_> = first_line.chars().skip(1).collect();
        let width = tiles.len();
        let mut expected_length = width;

        let mut height = 1;
        for line in lines {
            tiles.extend(line.chars());
            height += 1;

            expected_length += width;
            assert_eq!(tiles.len(), expected_length);
        }
        Some(Self {
            tiles,
            width,
            height,
        })
    }

    fn get_index(&self, point: Point2D) -> Option<usize> {
        if point.0 > self.width {
            return None;
        }
        let index = point.0 + (self.width * point.1);
        if index < self.tiles.len() {
            Some(index)
        } else {
            None
        }
    }

    fn get(&self, point: Point2D) -> Option<&char> {
        self.get_index(point).map(|i| self.tiles.get(i)).flatten()
    }
    
    fn is_point_inside(&self, point: Point2D) -> bool {
        point.0 < self.width && point.1 < self.height
    }
    
    fn get_beam_positions(&self, from: Beam) -> HashSet<Point2D> {
        let mut visited = HashSet::default();
        let mut splitters = HashSet::default();
        self.get_beam_positions_cached(from, &mut visited, &mut splitters);
        visited
    }

    fn get_beam_positions_cached(&self, from: Beam, visited: &mut HashSet<Point2D>,
                                 splitters: &mut HashSet<Point2D>,
    ) {
        let mut cur_beam = from;
        loop {
            if !self.is_point_inside(cur_beam.0) {
                break;
            }
            
            visited.insert(cur_beam.0);
            
            let tile = match self.get(cur_beam.0) {
                Some(tile) => tile,
                None => break,
            };
            match tile {
                '|' => match cur_beam.1 {
                    Direction::West | Direction::East => {
                        self.split_beam(cur_beam, visited, splitters);
                        break;
                    },
                    _ => {}
                },
                '-' => match cur_beam.1 {
                    Direction::North | Direction::South => {
                        self.split_beam(cur_beam, visited, splitters);
                        break;
                    },
                    _ => {}
                },
                '/' => {
                    let towards = match cur_beam.1 {
                        Direction::North | Direction::South => cur_beam.1.turn_right(),
                        Direction::East | Direction::West => cur_beam.1.turn_left(),
                    };
                    if let Some(new_beam) = cur_beam.try_move_one_towards(towards) {
                        // println!("{:?} hit a / mirror, now it's {:?}", cur_beam, new_beam);
                        self.get_beam_positions_cached(new_beam, visited, splitters);
                    }
                    break;
                },
                '\\' => {
                    let towards = match cur_beam.1 {
                        Direction::North | Direction::South => cur_beam.1.turn_left(),
                        Direction::East | Direction::West => cur_beam.1.turn_right(),
                    };
                    if let Some(new_beam) = cur_beam.try_move_one_towards(towards) {
                        // println!("{:?} hit a \\ mirror, now it's {:?}", cur_beam, new_beam);
                        self.get_beam_positions_cached(new_beam, visited, splitters);
                    }
                    break;
                },
                '.' => {
                    
                },
                _ => panic!("Invalid tile: {}", tile),
            }
            
            cur_beam = match cur_beam.try_move_one() {
                Some(beam) => beam,
                None => break,
            };
        }
    }

    fn split_beam(&self, from: Beam, visited: &mut HashSet<Point2D>,
                                 splitters: &mut HashSet<Point2D>,
    ) {
        if splitters.contains(&from.0) {
            // println!("Stopped splitters loop at {from:?}");
            return;
        }
        splitters.insert(from.0);
        
        if let Some(left_beam) = from.try_move_one_towards(from.1.turn_left()) {
            self.get_beam_positions_cached(left_beam, visited, splitters);
        }

        if let Some(right_beam) = from.try_move_one_towards(from.1.turn_right()) {
            self.get_beam_positions_cached(right_beam, visited, splitters);
        }
    }
}
