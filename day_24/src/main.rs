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
    // println!("{:#?}", hailstorm.hailstones);

    let rock = hailstorm.calculate_intersection_body();
    println!("Rock estimated to be {}", rock);
    let product: i64 = rock.position.iter().cloned().sum();
    println!("Result: {:?}", product);
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_24/input_test.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn exact_division(vector: Vector3<i64>, divisor: i64) -> Option<Vector3<i64>> {
    if vector.x % divisor != 0 || vector.y % divisor != 0 || vector.z % divisor != 0 {
        return None;
    }
    Some(Vector3::new(vector.x / divisor, vector.y / divisor, vector.z / divisor))
}

fn try_to_get_dimension_intersection_time(a_pos: i64, a_vel: i64, b_pos: i64, b_vel: i64) -> Option<i64> {
    let vel_diff = a_vel - b_vel;
    if vel_diff == 0 {
        return None;
    }
    let pos_diff = b_pos - a_pos;
    if pos_diff % vel_diff != 0 {
        return None;
    }
    let result = pos_diff / vel_diff;
    // println!("{result}");
    Some(result)
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
    
    fn from_velocity(velocity: Vector3<i64>) -> Self {
        Self::new(Vector3::zeros(), velocity)
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
    
    fn try_to_get_intersection_time(&self, other: &Self) -> Option<i64> {
        let x_result = try_to_get_dimension_intersection_time(
            self.position.x, self.velocity.x, other.position.x, other.velocity.x,
        );
        let y_result = try_to_get_dimension_intersection_time(
            self.position.y, self.velocity.y, other.position.y, other.velocity.y,
        );
        let z_result = try_to_get_dimension_intersection_time(
            self.position.z, self.velocity.z, other.position.z, other.velocity.z,
        );
        match (x_result, y_result, z_result) {
            (Some(x), Some(y), Some(z)) if x == y && y == z => Some(x),
            _ => None,
        }
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
    
    fn clone_relative_to(&self, hailstone: &Hailstone) -> Self {
        let mut new = self.clone();
        for h in new.hailstones.iter_mut() {
            h.position -= hailstone.position;
            h.velocity -= hailstone.velocity;
        }
        new
    }
    
    fn calculate_intersection_body(&self) -> Hailstone {
        let relative_hailstone = &self.hailstones[0];
        let relative_space = self.clone_relative_to(relative_hailstone);

        let target_hailstone = &relative_space.hailstones[1];
        let test_hailstone = &relative_space.hailstones[1];
        
        const LIMIT: i64 = 5;
        for relative_time in 1..=LIMIT {
            let target_position = target_hailstone.position_at(relative_time);
            println!("Time {}: {:?}", relative_time, target_position);
            
            let lowest_factor = lowest_factor_candidate(target_position);
            let possible_velocities = (1..=lowest_factor).flat_map(|i| try_divide(target_position, i));
            for velocity in possible_velocities {
                println!("{:?}", velocity);
            }
        }
        
        Hailstone::new(Vector3::zeros(), Vector3::zeros())
    }
}

fn lowest_factor_candidate(vector: Vector3<i64>) -> i64 {
    vector.iter().cloned().filter(|&n| n != 0).map(i64::abs).min().unwrap_or(1)
}

fn try_divide(v: Vector3<i64>, divisor: i64) -> Option<Vector3<i64>> {
    if v.x % divisor != 0 || v.y % divisor != 0 || v.z % divisor != 0 {
        return None;
    }
    Some(v / divisor)
}
