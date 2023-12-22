use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra::Vector3;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    sequence::tuple,
    IResult,
};

fn main() {
    part_1();
}

fn part_1() {
    let mut bricks = read_bricks(read_file());
    bricks.sort_by_key(|b| b.pos.z);
    println!("{bricks:?}");
}

fn read_bricks(reader: impl BufRead) -> Vec<Brick> {
    let mut bricks = vec![];
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (_, b) = Brick::parse_str(&line).unwrap();
        bricks.push(b);
    }
    bricks
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_22/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Debug, Clone)]
pub struct Brick {
    pos: Vector3<usize>,
    size: Vector3<usize>,
}

impl Brick {
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
}
