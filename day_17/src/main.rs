use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use aoc_shared::coords2d::Coords2D;
use aoc_shared::map2d::Map2D;

fn main() {
    let path = std::env::current_dir().unwrap().join("day_17/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    
    let map = TileMap::try_from_reader(&mut reader).unwrap().unwrap();
    println!("{}", map);
    
    part_1(&map);
}

fn part_1(map: &TileMap) {
    let start = Coords2D::ZERO;
    let destination = Coords2D(map.width(), map.height());
    pathfind_map(&map, start, destination);
}

fn pathfind_map(map: &TileMap, start: Coords2D, destination: Coords2D) {
    println!("Travelling from {:?} to {:?}. Distance: {}", start, destination, start.manhattan_distance_to(destination));

    let mut closed_list = HashSet::<Coords2D>::default();
    let mut open_list = HashSet::<Coords2D>::default();
}

struct Movement {
    from: Coords2D,
    to: Coords2D,
}

struct CalculatedCost {
    cost: usize,
    distance_left: usize,
}

impl CalculatedCost {
    fn heuristic_cost(&self) -> usize {
        self.cost + self.distance_left
    }
}

#[derive(Debug)]
struct Tile {
    cost: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cost)
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const RADIX: u32 = 10;
        let cost = value.to_digit(RADIX).ok_or("Expected a digit")?;
        Ok(Self { cost: cost as usize })
    }
}

type TileMap = Map2D<Tile>;
