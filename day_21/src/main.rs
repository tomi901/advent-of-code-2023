use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::DIRECTIONS;
use aoc_shared::map2d::CharMap;

fn main() {
    part_1();
}

fn part_1() {
    let mut file = read_file();
    let map = TileMap::from_reader(&mut file);

    let starting_position = map.find_starting_position().unwrap();
    println!("Starting at {:?}", starting_position);

    let mut positions = HashSet::default();
    positions.insert(starting_position);

    for i in 1..=64 {
        positions = map.get_next_positions(&positions);
        println!("{} step/s = {} positions", i, positions.len());
        // println!("{:?}", positions);
    }
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_21/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

struct TileMap(CharMap);

impl TileMap {
    pub fn from_reader(reader: &mut impl BufRead) -> Self {
        let map = CharMap::try_from_reader(reader).unwrap().unwrap();
        Self(map)
    }

    pub fn find_starting_position(&self) -> Option<Coords2D> {
        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                let point = Coords2D(x, y);
                match self.0.get(point) {
                    Some('S') => return Some(point),
                    _ => {},
                }
            }
        }
        None
    }

    pub fn get_next_positions(&self, current_positions: &HashSet<Coords2D>) -> HashSet<Coords2D> {
        let mut next_positions = HashSet::with_capacity(current_positions.len());
        for &pos in current_positions {
            for direction in DIRECTIONS {
                let next_possible = pos
                    .try_move_one(direction)
                    .and_then(|p| self.0.get(p).is_some_and(|&c| c != '#').then_some(p));
                if let Some(pos) = next_possible {
                    next_positions.insert(pos);
                }
            }
        }
        next_positions
    }
}
