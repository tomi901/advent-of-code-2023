use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::Direction;
use aoc_shared::map2d::Map2D;

fn main() {
    part_1();
}

fn part_1() {
    let map = TileMap::try_from_reader(&mut read_file());
    let start = map.get_start_position().unwrap();
    let end = map.get_end_position().unwrap();
    println!("Trying to find path between {:?} -> {:?}", start, end);

    let builder = thread::Builder::new()
        .name("Pathfinder".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(move || {
        // stack-intensive operations
        map.find_longest_path(start, Direction::South, end)
    }).unwrap();

    let result = handler.join().unwrap();
    println!("Result: {result:?}");
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_23/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Ground,
    Forest,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::North)),
            '>' => Ok(Self::Slope(Direction::East)),
            'v' => Ok(Self::Slope(Direction::South)),
            '<' => Ok(Self::Slope(Direction::West)),
            _ => Err(value),
        }
    }
}

impl Tile {
    fn is_walkable(&self, direction: Direction) -> bool {
        // println!("Checking {:?} with {:?}", self, direction);
        match self {
            Tile::Ground => true,
            Tile::Forest => false,
            Tile::Slope(d) => *d == direction, 
        }
    }
}

struct TileMap(Map2D<Tile>);

impl TileMap {
    fn try_from_reader(reader: &mut impl BufRead) -> Self {
        let map = Map2D::try_from_reader(reader).unwrap().unwrap();
        Self(map)
    }
    
    fn get_start_position(&self) -> Option<Coords2D> {
        (0..self.0.width())
            .map(|x| Coords2D(x, 0))
            .find(|&point| self.0.get(point).is_some_and(|t| t == &Tile::Ground))
    }

    fn get_end_position(&self) -> Option<Coords2D> {
        let last_y = self.0.height() - 1;
        (0..self.0.width())
            .map(|x| Coords2D(x, last_y))
            .find(|&point| self.0.get(point).is_some_and(|t| t == &Tile::Ground))
    }
    
    fn find_longest_path(
        &self, from: Coords2D, direction: Direction, destination: Coords2D,
    ) -> Option<usize> {
        // println!("Starting path from {:?} towards {:?}", from, direction);
        let mut cur_pos = from;
        let mut steps = 0;
        let left = direction.turn_left();
        let right = direction.turn_right();
        loop {
            // println!("- {:?}", cur_pos);
            if cur_pos == destination {
                return Some(steps);
            }

            // Here the paths potentially branch and differ
            let left_walk = self.get_walkable_tile_from(cur_pos, left);
            let right_walk = self.get_walkable_tile_from(cur_pos, right);
            if left_walk.is_some() || right_walk.is_some() {
                let branches = [
                    left_walk.map(|branch| (left, branch)),
                    right_walk.map(|branch| (right, branch)),
                    cur_pos.try_move_one(direction).map(|branch| (direction, branch)),
                ];
                // println!("Branching: {:?}", branches);
                return branches
                    .iter()
                    .flatten()
                    .map(|(d, b)| self.find_longest_path(*b, *d, destination))
                    .flatten()
                    .max()
                    .map(|max_branch| max_branch + steps + 1)
            }
            
            if !self.is_walkable(cur_pos, direction) {
                return None;
            }
            
            cur_pos = match cur_pos.try_move_one(direction) {
                Some(p) => p,
                None => return None,
            };
            steps += 1;
        }
    }

    fn get_walkable_tile_from(&self, point: Coords2D, direction: Direction) -> Option<Coords2D> {
        point.try_move_one(direction)
            .and_then(|p| self.is_walkable(point, direction).then_some(p))
    }
    
    fn is_walkable(&self, point: Coords2D, direction: Direction) -> bool {
        self.0.get(point).is_some_and(|t| t.is_walkable(direction))
    }
}
