use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use aoc_shared::coords2d::Coords2D;
use aoc_shared::direction::DIRECTIONS;
use aoc_shared::map2d::CharMap;

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let mut file = read_file();
    let map = TileMap::from_reader(&mut file);
    println!("Map is {}x{}", map.0.width(), map.0.height());

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

fn part_2() {
    let mut file = read_file();
    let map = TileMap::from_reader(&mut file);
    println!("Map is {}x{}", map.0.width(), map.0.height());

    let count = map.get_infinite_map_tiles_count(26501365);
    println!("Result: {}", count);
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_21/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Default)]
struct OddEven {
    odd: usize,
    even: usize,
}

impl OddEven {
    pub fn get_mut(&mut self, i: usize) -> &mut usize {
        if i % 2 == 0 {
            &mut self.even
        } else {
            &mut self.odd
        }
    }
    
    pub fn total(&self) -> usize {
        self.odd + self.even
    }
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
    
    pub fn get_infinite_map_tiles_count(&self, steps: usize) -> usize {
        println!("Calculating {} step/s", steps);
        
        if self.0.width() != self.0.height() {
            panic!("Non-squared maps not supported");
        }
        let map_length = self.0.width();

        // This assumes the tilemap loops and the starting point is at the middle
        let starting_position = self.find_starting_position().expect("No starting point");
        println!("Starting at {:?}", starting_position);

        let x_center = self.0.width() - ((self.0.width() + 1) / 2);
        let y_center = self.0.height() - ((self.0.height() + 1) / 2);
        if starting_position.0 != x_center || starting_position.1 != y_center {
            panic!("Not starting at center {:?}", (x_center, y_center));
        }

        let excess_steps = steps - x_center;
        if excess_steps % map_length != 0 {
            panic!("Not a perfect loop!")
        }
        let loops = excess_steps / map_length;
        println!("Loops: {}", loops);
        if loops % 2 != 0 {
            panic!("Loops have to be even");
        }
        
        let OddEven { odd, even } = self.get_possible_odd_and_even_tiles(starting_position);
        let odd_maps = (loops - 1) * (loops - 1);
        let even_maps = loops * loops;
        println!("Odd map count: {}", odd_maps);
        println!("Even map count: {}", even_maps);
        
        // 616583511703756
        // 616583483179597
        // 616583481763430
        
        let tips_count = self.get_possible_tips_tiles(starting_position);
        let small_corners_count = self.get_possible_small_corners_tiles(starting_position);
        let big_corners_count = self.get_possible_big_corners_tiles(starting_position);

        (odd_maps * odd) + (even_maps * even) + tips_count + (small_corners_count * loops) +
            (big_corners_count * (loops - 1))
    }
    
    fn get_possible_tiles_from(&self, point: Coords2D, steps: usize) -> HashSet<Coords2D> {
        let mut positions = HashSet::default();
        positions.insert(point);
        for _ in 1..=steps {
            positions = self.get_next_positions(&positions);
        }
        positions
    }
    
    fn get_possible_odd_and_even_tiles(&self, starting_point: Coords2D) -> OddEven {
        let mut positions = HashSet::default();
        positions.insert(starting_point);
        let mut counts = OddEven::default();
        let mut i = 0;
        loop {
            positions = self.get_next_positions(&positions);
            i += 1;
            let count = positions.len();

            let selected_count = counts.get_mut(i);
            if *selected_count == count {
                break;
            }
            *selected_count = count;
        }
        counts
    }
    
    fn get_possible_tips_tiles(&self, starting_point: Coords2D) -> usize {
        let length = self.0.width() - 1;
        let top = self.get_possible_tiles_from(Coords2D(starting_point.0, length), length);
        let right = self.get_possible_tiles_from(Coords2D(0, starting_point.1), length);
        let bottom = self.get_possible_tiles_from(Coords2D(starting_point.0, 0), length);
        let left = self.get_possible_tiles_from(Coords2D(length, starting_point.1), length);
        
        // self.display_tiles(&top);
        // self.display_tiles(&right);
        // self.display_tiles(&bottom);
        // self.display_tiles(&left);

        top.len() + right.len() + bottom.len() + left.len()
    }

    fn get_possible_small_corners_tiles(&self, starting_point: Coords2D) -> usize {
        let map_max = self.0.width() - 1;
        let length = starting_point.0 - 1;
        let top_right = self.get_possible_tiles_from(Coords2D(map_max, 0), length);
        let bottom_right = self.get_possible_tiles_from(Coords2D(map_max, map_max), length);
        let bottom_left = self.get_possible_tiles_from(Coords2D(0, map_max), length);
        let top_left = self.get_possible_tiles_from(Coords2D(0, 0), length);
        
        // self.display_tiles(&top_right);
        // self.display_tiles(&bottom_right);
        // self.display_tiles(&bottom_left);
        // self.display_tiles(&top_left);
        
        top_right.len() + bottom_right.len() + bottom_left.len() + top_left.len()
    }

    fn get_possible_big_corners_tiles(&self, starting_point: Coords2D) -> usize {
        let map_max = self.0.width() - 1;
        let length = starting_point.0 + map_max;
        let top_right = self.get_possible_tiles_from(Coords2D(map_max, 0), length);
        let bottom_right = self.get_possible_tiles_from(Coords2D(map_max, map_max), length);
        let bottom_left = self.get_possible_tiles_from(Coords2D(0, map_max), length);
        let top_left = self.get_possible_tiles_from(Coords2D(0, 0), length);
        
        // self.display_tiles(&top_right);
        // self.display_tiles(&bottom_right);
        // self.display_tiles(&bottom_left);
        // self.display_tiles(&top_left);
        
        top_right.len() + bottom_right.len() + bottom_left.len() + top_left.len()
    }
    
    pub fn display_tiles(&self, points: &HashSet<Coords2D>) {
        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                let point = Coords2D(x, y);
                if points.contains(&point) {
                    print!("\u{1b}[31;1m");
                    print!("O");
                    print!("\u{1b}[0m");
                } else {
                    print!("\u{1b}[2m");
                    print!("{}", self.0.get(point).unwrap());
                    print!("\u{1b}[0m");
                }
            }
            println!();
        }
    }
}
