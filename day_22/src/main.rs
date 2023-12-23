use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::iproduct;
use nalgebra::Vector3;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    sequence::tuple,
    IResult,
};
use nom::character::complete::char;
use nom::combinator::opt;

fn main() {
    part_1();
}

fn part_1() {
    let mut bricks = BrickMap::from_reader(read_file());
    println!("{:?}", bricks.positions().collect::<Vec<_>>());
    bricks.apply_gravity();
    println!("{:?}", bricks.positions().collect::<Vec<_>>());
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_22/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug)]
pub struct BrickMap {
    bricks: Vec<Bounds>,
    occupied_spots: HashSet<Vector3<usize>>,
}

impl BrickMap {
    fn from_reader(reader: impl BufRead) -> Self {
        let mut bricks = vec![];
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let (_, b) = Bounds::parse_str(&line).unwrap();
            bricks.push(b);
        }
        
        let mut occupied_spots = bricks
            .iter()
            .flat_map(|b| b.all_points())
            .collect();
        Self {
            bricks,
            occupied_spots,
        }
    }
    
    fn apply_gravity(&mut self) {
        self.sort_bricks();
        let mut occupied = self.occupied_spots.clone();
        for brick in self.bricks.iter_mut() {
            brick.all_points().for_each(|p| { occupied.remove(&p); });
            
            let mut check_area = brick.get_below_area();
            while check_area.pos.z > 0 && !check_area.collides_with_any(&occupied) {
                check_area.pos.z -= 1;
            }
            brick.pos.z = check_area.pos.z + 1;

            occupied.extend(brick.all_points());
        }
        self.occupied_spots = occupied;
        self.sort_bricks();
    }
    
    fn sort_bricks(&mut self) {
        self.bricks.sort_by_key(|b| b.pos.z);
    }
    
    fn positions(&self) -> impl Iterator<Item = Vector3<usize>> + '_ {
        self.bricks.iter().map(|b| b.pos)
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pos: Vector3<usize>,
    size: Vector3<usize>,
}

impl Bounds {
    fn from_to(from: Vector3<usize>, to: Vector3<usize>) -> Self {
        let size = to - from + Vector3::new(1, 1, 1);
        Self {
            pos: from,
            size,
        }
    }

    fn min(&self) -> Vector3<usize> {
        self.pos
    }

    fn max(&self) -> Vector3<usize> {
        self.pos + self.size
    }

    fn move_towards(&mut self, v: Vector3<usize>) {
        self.pos += v;
    }

    fn parse_str(s: &str) -> IResult<&str, Self> {
        let (s, _) = opt(char('\u{feff}'))(s)?;
        let mut parser = tuple((Self::parse_vector, tag("~"), Self::parse_vector));
        let (remaining, (from, _, to)) = parser(s)?;
        Ok((remaining, Self::from_to(from, to)))
    }

    fn parse_vector(s: &str) -> IResult<&str, Vector3<usize>> {
        let mut parser = tuple((digit1, tag(","), digit1, tag(","), digit1));
        let (remaining, (x, _, y, _, z)) = parser(s)?;
        let vector = Vector3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        Ok((remaining, vector))
    }
    
    fn all_points(&self) -> impl Iterator<Item = Vector3<usize>> {
        let from = self.min();
        let to = self.max();
        iproduct!(from.x..to.x, from.y..to.y, from.z..to.z)
            .map(|(x, y, z)| Vector3::new(x, y, z))
    }
    
    fn get_below_area(&self) -> Self {
        let mut new = self.clone();
        new.pos.z -= 1;
        new.size.z = 1;
        new
    }

    fn collides_with_any(&self, occupied_spots: &HashSet<Vector3<usize>>) -> bool {
        if self.pos.z <= 0 {
            return false;
        }
        self.all_points().any(|p| occupied_spots.contains(&p))
    }
}
