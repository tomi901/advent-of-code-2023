use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra::Vector3;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, one_of, space0};
use nom::combinator::{opt, recognize};
use nom::multi::many0;
use nom::sequence::tuple;

fn main() {
    part_1();
}

fn part_1() {
    let hailstorm = Hailstorm::from_reader(&mut read_file());
    println!("{:#?}", hailstorm);
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_24/input_test.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

#[derive(Clone)]
pub struct Hailstone {
    position: Vector3<i64>,
    velocity: Vector3<i64>,
}

impl Hailstone {
    fn new(position: Vector3<i64>, velocity: Vector3<i64>) -> Self {
        Self {
            position,
            velocity,
        }
    }
    
    fn parse_str(s: &str) -> IResult<&str, Self> {
        let (s, _) = opt(char('\u{feff}'))(s)?;
        let mut parser = tuple((
            Self::parse_vector,
            space0,
            tag("@"),
            space0,
            Self::parse_vector,
        ));
        let (remaining, (position, _, _, _, velocity)) = parser(s)?;
        Ok((remaining, Self::new(position, velocity)))
    }

    fn parse_vector(s: &str) -> IResult<&str, Vector3<i64>> {
        let mut parser = tuple((
            Self::parse_number,
            tag(","),
            space0,
            Self::parse_number,
            tag(","),
            space0,
            Self::parse_number,
        ));
        let (remaining, (x, _, _, y, _, _, z)) = parser(s)?;
        let vector = Vector3::new(x, y, z);
        Ok((remaining, vector))
    }
    
    fn parse_number(s: &str) -> IResult<&str, i64> {
        let mut parser = recognize(tuple((
            opt(one_of("+-")),
            digit1,
        )));
        let (remaining, num) = parser(s)?;
        Ok((remaining, num.parse().unwrap()))
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}", self.position.x, self.position.y, self.position.z,
               self.velocity.x, self.velocity.y, self.velocity.z)
    }
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    slope: f64,
    bias: f64,
}

#[derive(Debug, Clone)]
pub struct Hailstorm {
    hailstones: Vec<Hailstone>,
}

impl Hailstorm {
    fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut hailstones = vec![];
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let (_, new) = Hailstone::parse_str(&line).unwrap();
            hailstones.push(new);
        }
        Self {
            hailstones,
        }
    }
}
