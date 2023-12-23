use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let mut map = BrickMap::from_reader(read_file());
    map.sort_bricks();
    // println!("{:?}", map.positions().collect::<Vec<_>>());
    map.apply_gravity();
    // for brick in &map.bricks {
    //     println!("{}", brick);
    // }
    // println!("{:?}", map.positions().collect::<Vec<_>>());
    
    let deletable = map.calculate_deletable_bricks();
    println!("Deletable: {}", deletable.len());
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_22/input_cached.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug)]
pub struct BrickMap {
    bricks: Vec<Bounds>,
}

impl BrickMap {
    fn from_reader(reader: impl BufRead) -> Self {
        let mut bricks = vec![];
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let (_, b) = Bounds::parse_str(&line).unwrap();
            bricks.push(b);
        }
        Self {
            bricks,
        }
    }
    
    fn apply_gravity(&mut self) {
        // Too many loops, but some bricks ended suspended in the air
        // Find a better a better algorithm
        loop {
            println!("Gravity iteration");
            let mut new_positions: Vec<(usize, usize)> = vec![];
            
            for (i, brick) in self.bricks.iter().enumerate() {
                if brick.pos.z <= 1 {
                    continue;
                }
                
                let mut check_area = brick.get_below_area();
                // println!("Starting cast for {} from {}", brick, check_area);
                
                let mut moved = false;
                while check_area.pos.z > 0 && self.check_collisions(check_area).next().is_none() {
                    check_area.pos.z -= 1;
                    moved = true;
                }
                
                if moved {
                    // println!("Lowering brick {} ({:?}): {} -> {}", i, brick, brick.pos.z, check_area.pos.z + 1);
                    new_positions.push((i, check_area.pos.z + 1));
                }
            }
            
            if new_positions.is_empty() {
                break;
            }

            for (i, pos) in new_positions {
                // println!("Moving {} to {}", i, pos);
                self.bricks[i].pos.z = pos;
            }
        }
    }
    
    fn check_collisions<'a>(&'a self, area: Bounds) -> impl Iterator<Item = &'a Bounds> + 'a {
        self.bricks.iter().filter(move |&b| b.check_aabb(area))
    }
    
    fn sort_bricks(&mut self) {
        self.bricks.sort_by_key(|b| b.pos.z);
    }
    
    fn positions(&self) -> impl Iterator<Item = Vector3<usize>> + '_ {
        self.bricks.iter().map(|b| b.pos)
    }
    
    fn calculate_deletable_bricks(&self) -> HashSet<usize> {
        let mut deletable = (0..self.bricks.len()).collect::<HashSet<_>>();
        for (i, brick) in self.bricks.iter().enumerate() {
            if brick.pos.z <= 1 {
                // println!("Brick {} supported by ground", i);
                continue;
            }
            
            let check_area = brick.get_below_area();
            let supporters = self.bricks.iter()
                .enumerate()
                .filter(|(i, &b)| check_area.check_aabb(b))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            // println!("Brick {} {:?} supported by:", i, brick);
            // println!("{:?}", supporters);
            
            if supporters.is_empty() {
                panic!("Brick suspended in air!");
            }
            
            if supporters.len() == 1 {
                deletable.remove(&supporters[0]);
            }
        }
        deletable
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pos: Vector3<usize>,
    extend: Vector3<usize>,
}

impl Bounds {
    fn from_to(from: Vector3<usize>, to: Vector3<usize>) -> Self {
        let extend = to - from;
        Self {
            pos: from,
            extend,
        }
    }

    fn min(&self) -> Vector3<usize> {
        self.pos
    }

    fn max(&self) -> Vector3<usize> {
        self.pos + self.extend
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
    
    fn get_below_area(&self) -> Self {
        let mut new = self.clone();
        new.pos.z -= 1;
        new.extend.z = 0;
        new
    }
    
    fn check_aabb(&self, other: Self) -> bool {
        let self_max = self.max();
        let other_max = other.max();
        // println!("Checking {:?} <=> {:?}", (self.pos, self_max), (other.pos, other_max));
        if self.pos.x > other_max.x || other.pos.x > self_max.x {
            return false;
        }
        if self.pos.y > other_max.y || other.pos.y > self_max.y {
            return false;
        }
        if self.pos.z > other_max.z || other.pos.z > self_max.z {
            return false;
        }
        return true;
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let from = self.min();
        let to = self.max();
        write!(f, "{},{},{}~{},{},{}", from.x, from.y, from.z, to.x, to.y, to.z)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use crate::Bounds;

    #[test]
    fn check_abb_works_returns_true_correctly() {
        let a = Bounds::from_to(Vector3::new(1,0,1), Vector3::new(1,2,1));
        let b = Bounds::from_to(Vector3::new(0,1,1), Vector3::new(5,1,1));
        
        let result = a.check_aabb(b);
        
        assert_eq!(result, true);
    }

    #[test]
    fn check_abb_works_returns_false_correctly() {
        let a = Bounds::from_to(Vector3::new(1,0,1), Vector3::new(1,2,1));
        let b = Bounds::from_to(Vector3::new(1,1,2), Vector3::new(1,1,2));

        let result = a.check_aabb(b);

        assert_eq!(result, false);
    }
}
