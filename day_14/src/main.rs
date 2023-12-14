use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = std::env::current_dir().unwrap().join("day_14/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut map = Map::try_from(&mut reader).unwrap();
    println!("Starting map:");
    println!("{}", &map);
    println!();
    
    map.tilt_north();
    println!("Tilted map:");
    println!("{}", &map);
    println!();

    let load = map.calculate_north_load();
    println!("Load: {}", load);
}

type Tile = char;
const ROUND_ROCK: Tile = 'O';
const CUBE_ROCK: Tile = '#';
const GROUND: Tile = '.';

#[derive(Debug, Copy, Clone, Default)]
struct Point2D(usize, usize);

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
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
        let index = point.0 + (self.width * point.1);
        if index < self.tiles.len() {
            Some(index)
        } else {
            None
        }
    }

    fn get(&self, point: Point2D) -> Option<&Tile> {
        self.get_index(point).map(|i| self.tiles.get(i)).flatten()
    }
    
    fn calculate_north_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point2D(x, y);
                let tile = self.get(point).unwrap();
                if *tile != ROUND_ROCK {
                    continue;
                }

                let distance = self.height - y;
                load += distance;
            }
        }
        load
    }

    fn tilt(&mut self, direction: Direction) {
        for x in 0..self.width {
            let mut cur_sequence = 0;
            for y in (0..self.height).rev() {
                let point = Point2D(x, y);
                let index = self.get_index(point).expect("Out of range");
                match self.tiles[index] {
                    CUBE_ROCK if (y + 1) < self.height => {
                        let from = y + 1;
                        for previous_y in from..(from + cur_sequence) {
                            let index = self.get_index(Point2D(x, previous_y)).unwrap();
                            self.tiles[index] = ROUND_ROCK;
                        }
                        cur_sequence = 0;
                    },
                    ROUND_ROCK => {
                        self.tiles[index] = GROUND;
                        cur_sequence += 1;
                    },
                    _ => {}
                }
            }

            if cur_sequence > 0 {
                for previous_y in 0..cur_sequence {
                    let index = self.get_index(Point2D(x, previous_y)).unwrap();
                    self.tiles[index] = ROUND_ROCK;
                }
            }
        }
    }
}

struct ScanLine<'a> {
    map: &'a Map,
    number: usize,
    direction: Direction,
}

impl ScanLine {
    /// Gets the point at the scan line index
    fn get_point(&self, i: usize) -> Point2D {
        match self.direction {
            Direction::North => Point2D(self.number, self.map.height - i),
            Direction::East => Point2D(i, self.number),
            Direction::South => Point2D(self.number, i),
            Direction::West => Point2D(self.map.width - i, self.number),
        }
    }
    
    fn len(&self) -> usize {
        match self.direction {
            Direction::North | Direction::South => self.map.height,
            Direction::East | Direction::West => self.map.width,
        }
    }
}
