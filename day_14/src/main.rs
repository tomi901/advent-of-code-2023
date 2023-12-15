use std::collections::hash_map::Entry;
use std::collections::HashMap;
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

    // part_1(map);
    part_2(map);
}

fn part_1(mut map: Map) {
    map.tilt(Direction::North);
    println!("Tilted map:");
    println!("{}", &map);
    println!();

    let load = map.calculate_north_load();
    println!("Load: {}", load);
}

fn part_2(mut map: Map) {
    const CYCLES: usize = 1000000000;
    const ITERATIONS: usize = 1000;
    
    // I was lazy to make an algorithm (And probably in a non foolproof way) that would
    // find the cycle and extrapolate from 1000000000.
    // This worked for my input.
    // However, probably storing the values in a vector is a better way.
    
    // (last_seen_index, estimated_length)
    let mut repeating_values_length = HashMap::<usize, (usize, Option<usize>)>::default();
    for i in 1..=ITERATIONS {
        map.tilt(Direction::North);
        map.tilt(Direction::West);
        map.tilt(Direction::South);
        map.tilt(Direction::East);

        let cur_value = map.calculate_north_load();
        match repeating_values_length.entry(cur_value) {
            Entry::Occupied(mut o) => {
                let previous = *o.get();
                o.insert((i, Some(i - previous.0)));
            },
            Entry::Vacant(mut v) => {
                v.insert((i, None));
            },
        }
        // print!("{load}, ");
    }
    println!();
    println!("{}", &map);
    println!();
    let max_length = repeating_values_length
        .values()
        .map(|x| x.1)
        .flatten()
        .max();
    // println!("Values:");
    // println!("{:#?}", repeating_values_length);
    println!("Max length: {:?}", max_length);
    
    let estimated_cycle = max_length.unwrap();
    let value = repeating_values_length
        .iter()
        .find(|(value, &info)| {
            ((CYCLES - info.0) % estimated_cycle) == 0
        })
        .map(|v| v.0);
    
    println!("Estimated value: {:?}", value);
    // let other_values: Vec<_> = repeating_values_length.keys().collect();
    // println!("Other values:\n{:#?}", other_values);
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
        let span = match direction {
            Direction::North | Direction::South => self.width,
            Direction::East | Direction::West => self.height,
        };
        for i in 0..span {
            let scan_line = ScanLine::new(&self, i, direction);
            let mut cur_sequence = 0;
            for j in 0..scan_line.len() {
                let point = scan_line.get_point(j);
                // println!("Scanning {:?}", point);
                let index = self.get_index(point).unwrap();
                let tile = self.tiles[index];
                match tile {
                    CUBE_ROCK => {
                        self.set_rocks(&scan_line, j, cur_sequence);
                        cur_sequence = 0;
                    },
                    ROUND_ROCK => {
                        cur_sequence += 1;
                        self.tiles[index] = GROUND;
                    },
                    _ => {}
                }
            }
            
            if cur_sequence > 0 {
                self.set_rocks(&scan_line, scan_line.len(), cur_sequence);
            }
        }
    }

    fn set_rocks(&mut self, scan_line: &ScanLine, until: usize, amount: usize) {
        for i in (until - amount)..until {
            let point = scan_line.get_point(i);
            let index = self.get_index(point).unwrap();
            self.tiles[index] = ROUND_ROCK;
        }
    }
}

struct ScanLine {
    len: usize,
    number: usize,
    direction: Direction,
}

impl ScanLine {
    fn new(map: &Map, number: usize, direction: Direction) -> Self {
        let len = match direction {
            Direction::North | Direction::South => map.height,
            Direction::East | Direction::West => map.width,
        };
        Self {
            len,
            number,
            direction,
        }
    }

    /// Gets the point at the scan line index
    fn get_point(&self, i: usize) -> Point2D {
        match self.direction {
            Direction::North => Point2D(self.number, self.len - i - 1),
            Direction::East => Point2D(i, self.number),
            Direction::South => Point2D(self.number, i),
            Direction::West => Point2D(self.len - i - 1, self.number),
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
}
