use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use aoc_shared::map2d::Map2D;
use byteorder::BigEndian;

fn main() {
    let path = std::env::current_dir().unwrap().join("day_17/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    
    let map = TileMap::try_from_reader(&mut reader).unwrap().unwrap();
    println!("{}", map);
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
