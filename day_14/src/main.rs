use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = std::env::current_dir().unwrap().join("day_14/input.txt");
    println!("Opening file {}", path.display());
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let map = Map::try_from(&mut reader).unwrap();
    println!("{}x{}", map.width, map.height);

    let load = map.calculate_north_load();
    println!("Load: {}", load);
}

type Tile = char;
const ROUND_ROCK: Tile = 'O';
const CUBE_ROCK: Tile = '#';
const SPACE: Tile = '.';

#[derive(Debug)]
struct Point2D(usize, usize);

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn try_from(input: &mut impl BufRead) -> Option<Self> {
        let mut lines = input.lines().flatten().take_while(|l| !l.is_empty());
        let first_line = match lines.next() {
            Some(line) => line,
            None => return None,
        };

        let mut tiles: Vec<_> = first_line.chars().collect();
        let width = tiles.len();
        let mut height = 1;
        for line in lines {
            tiles.extend(line.chars());
            height += 1;
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
}
