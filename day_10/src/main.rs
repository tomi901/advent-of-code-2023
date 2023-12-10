use std::cmp::min;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, stdin};

fn main() {
    let mut map = TileMap::parse(stdin().lock());
    let (start_point, start_pipe) = map.find_starting_point().expect("No starting point.");
    println!("{:?}", start_pipe);
    println!();
    
    println!("Starting from: {:?}", start_point);
    
    // let max_distance = map.find_furthest_distance(starting_point);
    // println!("Distance: {:?}", max_distance);
    
    let circuit = map.create_circuit(start_point);
    // println!("Circuit: {:?}", circuit);
    
    map.clean(&circuit);

    /*
    println!();
    for (_, row) in map.rows().enumerate() {
        for (_, tile) in row.into_iter().enumerate() {
            print!("{}", tile);
        }
        println!();
    }
    println!();
    */

    let mut inside_count = 0;
    for (y, row) in map.rows().enumerate() {
        let mut hit_count = 0;
        for (x, tile) in row.into_iter().enumerate() {
            let point: Point = (x as isize, y as isize);
            // println!("{:?}", point);
            if point == start_point {
                if start_pipe.has_direction(Direction::North) {
                    hit_count += 1;
                }
                continue;
            }
            match tile {
                Tile::None => {
                    let is_inside = (hit_count % 2) != 0;
                    if is_inside {
                        // println!("{:?} inside ({} hits)", point, hit_count);
                        inside_count += 1;
                    }
                },
                Tile::Start => {
                    hit_count += 1;
                },
                Tile::Pipe(pipe) => {
                    // We need to handle continuous horizontal pipe lines
                    if !circuit.contains(&point) {
                        let is_inside = (hit_count % 2) != 0;
                        if is_inside {
                            println!("{:?} inside ({} hits)", point, hit_count);
                            inside_count += 1;
                        }
                        continue;
                    }
                    
                    if pipe.has_direction(Direction::North) {
                        hit_count += 1;
                    }
                },
            }
        }
    }
    println!("Inside count: {}", inside_count);
}

type Point = (isize, isize);

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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
    
    fn has_directions(&self, direction1: Direction, direction2: Direction) -> bool {
        self.has_direction(direction1) && self.has_direction(direction2)
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

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = if self.has_directions(Direction::North, Direction::South) {
            '|'
        } else if self.has_directions(Direction::West, Direction::East) {
            '-'
        } else if self.has_directions(Direction::North, Direction::East) {
            'L'
        } else if self.has_directions(Direction::North, Direction::West) {
            'J'
        } else if self.has_directions(Direction::South, Direction::West) {
            '7'
        } else if self.has_directions(Direction::South, Direction::East) {
            'F'
        } else {
          panic!("Cannot display {:?}", &self);  
        };
        f.write_char(c)
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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::None => f.write_char('.'),
            Tile::Start => f.write_char('S'),
            Tile::Pipe(pipe) => pipe.fmt(f),
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
        let first_line = &lines.next().unwrap().unwrap();
        // println!("{}", &first_line);
        let mut tiles: Vec<Tile> = Vec::from_iter(Self::parse_line(first_line));
        let width = tiles.len();
        let mut height = 1;

        for line_result in lines {
            let line = line_result.unwrap();
            // println!("{}", &line);
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

    fn find_starting_point(&self) -> Option<(Point, Pipe)> {
        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                let tile = self.get_tile((x, y));
                if tile.is_some() && tile.unwrap().is_start() {
                    let point: Point = (x, y);
                    let mut directions = DIRECTIONS
                        .into_iter()
                        .filter(|&direction| {
                            let next_point = Direction::move_towards(point, direction);
                            let enter_from = direction.inverse();
                            match self.get_tile(next_point) {
                                Some(tile) => match tile {
                                    Tile::Pipe(pipe) if pipe.has_direction(enter_from) => true,
                                    _ => false,
                                },
                                _ => false,
                            }
                        });
                    let pipe = crate::Pipe {
                        a: directions.next().unwrap(),
                        b: directions.next().unwrap(),
                    };
                    return Some((point, pipe));
                }
            }
        }
        None
    }
    
    fn find_furthest_distance(&self, starting_point: Point) -> Option<usize> {
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
                self.travel_pipes(next_pos, towards, &mut distances);
            }
        }

        distances.into_values().max()
    }
    
    fn travel_pipes(&self, from_pos: Point, towards: Direction,
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

    fn create_circuit(&self, starting_point: Point) -> HashSet<Point> {
        // TODO: We could create an Iterator to travel to all pipes to follow the DRY pattern
        
        let (start_pos, _, start_direction) = DIRECTIONS.into_iter().map(|direction| {
            let next_pos = Direction::move_towards(starting_point, direction);
            let tile = self.get_tile(next_pos);
            if tile.is_none() {
                return None;
            }

            if let Tile::Pipe(pipe) = tile.unwrap() {
                // println!("Found tile at {:?} {:?}", next_pos, direction);
                let enter_from = direction.inverse();
                if !pipe.has_direction(enter_from) {
                    return None;
                }
                let towards = pipe.get_other_direction(enter_from).unwrap();
                return Some((next_pos, pipe, towards));
            }
            
            None
        }).flatten().next().unwrap();
        
        let mut circuit = HashSet::<Point>::new();
        circuit.insert(starting_point);
        circuit.insert(start_pos);
        
        let mut cur_pos = start_pos;
        let mut cur_direction = start_direction;
        loop {
            cur_pos = Direction::move_towards(cur_pos, cur_direction);
            let next_tile = self.get_tile(cur_pos).expect("Out of bounds.");
            if next_tile.is_start() {
                break;
            }

            let next_pipe = next_tile.try_get_pipe().expect("Next wasn't a pipe.");
            let enter_from = cur_direction.inverse();
            // println!("Will enter {:?} from {:?}", &next_pipe, enter_from);
            cur_direction = next_pipe
                .get_other_direction(enter_from)
                .expect("Error getting next direction.");

            circuit.insert(cur_pos);
        }
        circuit
    }
    
    fn rows(&self) -> impl Iterator<Item = &[Tile]> + '_ {
        (0..self.height).map(|y| {
            let from = self.width * y;
            &self.tiles[from..(from + self.width)]
        })
    }

    fn clean(&mut self, maintain: &HashSet<Point>) {
        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                let point = (x, y);
                if !maintain.contains(&point) {
                    let index = (point.0 as usize) + (self.width * (point.1 as usize));
                    self.tiles[index] = Tile::None;
                }
            }
        }
    }
}
