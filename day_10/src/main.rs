use std::cmp::min;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() {
    let map = TileMap::parse(stdin().lock());
    let starting_point = map.find_starting_point().expect("No starting point.");
    
    println!("Starting from: {:?}", starting_point);
    
    let max_distance = map.find_furthest_distance(starting_point);
    println!("Distance: {:?}", max_distance);
}

type Point = (isize, isize);

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
    
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    
    fn move_towards(from: Point, direction: Direction) -> Point {
        let offset = direction.to_point();
        (from.0 + offset.0, from.1 + offset.1)
    }
}

#[derive(PartialEq, Debug)]
struct Pipe {
    a: Direction,
    b: Direction,
}

impl Pipe {
    fn new(a: Direction, b: Direction) -> Self {
        Pipe { a, b }
    }
    
    fn try_from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Pipe::new(Direction::North, Direction::South)),
            '-' => Some(Pipe::new(Direction::West, Direction::East)),
            'L' => Some(Pipe::new(Direction::North, Direction::East)),
            'J' => Some(Pipe::new(Direction::North, Direction::West)),
            '7' => Some(Pipe::new(Direction::South, Direction::West)),
            'F' => Some(Pipe::new(Direction::South, Direction::East)),
            _ => None,
        }
    }
    
    fn has_direction(&self, direction: Direction) -> bool {
        self.a == direction || self.b == direction
    }
    
    fn get_other_direction(&self, direction: Direction) -> Option<Direction> {
        if self.a == direction {
            Some(self.b)
        } else if self.b == direction {
            Some(self.a)
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    None,
    Start,
    Pipe(Pipe),
}

impl Tile {
    fn from_char(c: char) -> Self {
        if let Some(pipe) = Pipe::try_from_char(c) {
            return Tile::Pipe(pipe);
        }
        
        match c {
            'S' => Tile::Start,
            '.' => Tile::None,
            _ => panic!("Couldn't parse char: {}", c),
        }
    }
    
    fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    } 
    
    fn try_get_pipe(&self) -> Option<&Pipe> {
        match self {
            Tile::Pipe(pipe) => Some(pipe),
            _ => None,
        }
    }
}

struct TileMap {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl TileMap {
    fn parse(input: impl BufRead) -> Self {
        let mut lines = input.lines();
        let mut tiles: Vec<Tile> = Vec::from_iter(
            Self::parse_line(&lines.next().unwrap().unwrap())
        );
        let width = tiles.len();
        let mut height = 1;

        for line_result in lines {
            let line = line_result.unwrap();
            tiles.extend(Self::parse_line(&line));

            height += 1;
        }

        assert_eq!(tiles.len(), width * height);
        Self {
            tiles,
            width,
            height,
        }
    }

    fn parse_line(s: &str) -> impl Iterator<Item = Tile> + '_ {
        s.chars().map(Tile::from_char)
    }

    fn get_tile(&self, point: Point) -> Option<&Tile> {
        if point.0 < 0 || point.1 < 0 {
            return None;
        }
        let index = (point.0 as usize) + (self.width * (point.1 as usize));
        self.tiles.get(index)
    }

    fn find_starting_point(&self) -> Option<Point> {
        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                let tile = self.get_tile((x, y));
                if tile.is_some() && tile.unwrap().is_start() {
                    return Some((x, y));
                }
            }
        }
        None
    }
    
    fn find_furthest_distance(&self, starting_point: Point) -> Option<usize> {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        
        let mut distances = HashMap::<Point, usize>::new();
        for direction in DIRECTIONS {
            let next_pos = Direction::move_towards(starting_point, direction);
            let tile = self.get_tile(next_pos);
            if tile.is_none() {
                continue;
            }
            
            if let Tile::Pipe(pipe) = tile.unwrap() {
                // println!("Found tile at {:?} {:?}", next_pos, direction);
                let enter_from = direction.inverse();
                if !pipe.has_direction(enter_from) {
                    continue;
                }
                let towards = pipe.get_other_direction(enter_from).unwrap();
                println!("Starting from {:?} {:?} -> {:?}", next_pos, pipe, towards);
                self.travel_pipes(next_pos, pipe, towards, &mut distances);
            }
        }

        distances.into_values().max()
    }
    
    fn travel_pipes(&self, from_pos: Point, from_pipe: &Pipe, towards: Direction,
                    distances: &mut HashMap<Point, usize>) {
        let mut cur_pos = from_pos;
        let mut cur_direction = towards;
        let mut cur_distance = 1;
        Self::set_distance(cur_pos, cur_distance, distances);
        
        loop {
            let next_pos = Direction::move_towards(cur_pos, cur_direction);
            let next_tile = self.get_tile(next_pos).expect("Out of bounds.");
            if next_tile.is_start() {
                break;
            }
            
            let next_pipe = next_tile.try_get_pipe().expect("Next wasn't a pipe.");
            let enter_from = cur_direction.inverse();
            // println!("Will enter {:?} from {:?}", &next_pipe, enter_from);
            let next_direction = next_pipe
                .get_other_direction(enter_from)
                .expect("Error getting next direction.");
            
            // println!("{:?} {:?} -> {:?} {:?} -> {:?}", cur_pos, cur_direction,
            //          next_pos, &next_pipe, next_direction);
            
            cur_pos = next_pos;
            cur_direction = next_direction;
            cur_distance += 1;
            
            Self::set_distance(cur_pos, cur_distance, distances);
        }
    }
    
    fn set_distance(point: Point, distance: usize, distances: &mut HashMap<Point, usize>) {
        match distances.entry(point) {
            Entry::Occupied(mut o) => {
                o.insert(min(distance, *o.get()));
            },
            Entry::Vacant(v) => {
                v.insert(distance);
            },
        }
    }
}
