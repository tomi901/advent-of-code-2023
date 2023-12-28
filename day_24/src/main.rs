use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use nalgebra::{Vector2, Vector3};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, one_of, space0};
use nom::combinator::{opt, recognize};
use nom::sequence::tuple;

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let hailstorm = Hailstorm::from_reader(&mut read_file());
    // println!("{:#?}", hailstorm);
    println!("{:#?}", hailstorm.hailstones.iter().map(Hailstone::as_xy_line).collect::<Vec<_>>());
    
    let range: RangeInclusive<i64> = 200000000000000..=400000000000000;
    let range_f64 = (*range.start() as f64)..=(*range.end() as f64);
    let crossings = hailstorm.get_crossings_count(&range_f64);
    println!("Result: {}", crossings);
}

fn part_2() {
    let hailstorm = Hailstorm::from_reader(&mut read_file());
    println!("{:#?}", hailstorm.hailstones);
    let relative = hailstorm.clone_relative_to_stone(0);
    println!("{:#?}", relative.hailstones);
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
    
    fn position_at(&self, time: i64) -> Vector3<i64> {
        self.position + (self.velocity * time)
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
    
    fn as_xy_line(&self) -> Line2D {
        let slope = (self.velocity.y as f64) / (self.velocity.x as f64);
        let bias = (self.position.y as f64) - (slope * (self.position.x as f64));
        let range = if self.velocity.x > 0 {
            (self.position.x as f64)..=f64::INFINITY
        } else {
            f64::NEG_INFINITY..=(self.position.x as f64)
        };
        Line2D {
            slope,
            bias,
            range,
        }
    }

    fn get_crossing(&self, other: &Self) -> Option<Vector2<f64>> {
        self.as_xy_line().get_crossing(&other.as_xy_line())
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

#[derive(Debug, Clone)]
pub struct Line2D {
    slope: f64,
    bias: f64,
    range: RangeInclusive<f64>,
}

impl Line2D {
    fn contains(&self, x: f64) -> bool {
        self.range.contains(&x)
    }
    
    fn value_at(&self, x: f64) -> Option<f64> {
        self.contains(x).then(|| (self.slope * x) + self.bias)
    }
    
    fn get_crossing(&self, other: &Self) -> Option<Vector2<f64>> {
        let slope_diff = self.slope - other.slope;
        let bias_diff = other.bias - self.bias;
        // println!("Slope diff: {slope_diff}");
        // println!("Bias diff: {bias_diff}");
        let cross_x = bias_diff / slope_diff;
        (self.contains(cross_x) && other.contains(cross_x))
            .then(|| Vector2::new(cross_x, self.value_at(cross_x).unwrap()))
    }
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
    
    fn get_crossings_count(&self, in_range: &RangeInclusive<f64>) -> usize {
        let mut count = 0;
        for i in 0..self.hailstones.len() {
            let a = &self.hailstones[i];
            for j in (i + 1)..self.hailstones.len() {
                let b = &self.hailstones[j];
                let crossing = a.get_crossing(b);
                // println!("Crossing ({i} - {j}) at: {crossing:?}");
                if crossing.is_some_and(|c| in_range.contains(&c.x) && in_range.contains(&c.y)) {
                    count += 1;
                }
            }
        }
        count
    }
    
    fn clone_relative_to_stone(&self, index: usize) -> Self {
        let relative_to = &self.hailstones[index];
        let mut new = self.clone();
        for hailstone in new.hailstones.iter_mut() {
            hailstone.position -= relative_to.position;
            hailstone.velocity -= relative_to.velocity;
        }
        new
    }
}
