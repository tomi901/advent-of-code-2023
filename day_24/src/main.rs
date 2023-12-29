use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::RangeInclusive;
use nalgebra::{Vector2, Vector3};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, one_of, space0};
use nom::combinator::{opt, recognize};
use nom::sequence::tuple;
use num_integer::Roots;

fn main() {
    // part_1();
    part_2();
    // let set = get_abs_factors(900).collect::<HashSet<_>>();
    // println!("Factors: ({}) {:?}", set.len(), &set);
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

// Answer based on:
// https://github.com/yongjun21/advent-of-code/blob/master/2023/day24.js#L2
// I tried to copy the least amount of code and tried to work on my own
fn part_2() {
    let hailstorm = Hailstorm::from_reader(&mut read_file());
    // println!("{:#?}", hailstorm.hailstones);

    let rock = hailstorm.calculate_start();
    println!("Rock estimated to be {}", rock);
    let product: i64 = rock.position.iter().cloned().sum();
    println!("Result: {:?}", product);
}

fn read_file() -> impl BufRead {
    let path = std::env::current_dir().unwrap().join("day_24/input.txt");
    println!("Opening file: {}", path.display());
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn get_abs_factors(n: i64) -> impl Iterator<Item = i64> {
    let n_abs = n.abs();
    let sqrt = n_abs.sqrt();
    let upper = sqrt + 1;
    // println!("{}", upper);
    (1..upper).filter_map(move |i| (n_abs % i == 0).then(|| [i, n_abs / i]))
        .flatten()
        .chain((upper * upper == n_abs).then_some(upper))
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
    
    fn add(&self, other: &Self) -> Self {
        Hailstone::new(self.position + other.position, self.velocity + other.velocity)
    }

    fn with_added_velocity(&self, velocity: Vector3<i64>) -> Self {
        Hailstone::new(self.position, self.velocity + velocity)
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
    
    fn calculate_start(&self) -> Hailstone {
        let mut result_candidates: [HashSet<i64, _>; 3] = [HashSet::default(), HashSet::default(), HashSet::default()];
        'outer: for i in 0..self.hailstones.len() {
            let a = &self.hailstones[i];
            for j in (i + 1)..self.hailstones.len() {
                let b = &self.hailstones[j];
                for dimension in 0..3 {
                    let ref mut cur_candidates = result_candidates[dimension];
                    if cur_candidates.len() == 1 {
                        continue;
                    }
                    
                    let velocity_diff = a.velocity[dimension] - b.velocity[dimension];
                    if velocity_diff != 0 {
                        continue;
                    }
                    
                    let velocity = a.velocity[dimension];
                    let position_diff = a.position[dimension] - b.position[dimension];
                    let mut new_candidates = get_abs_factors(position_diff)
                        .map(|f| [velocity + f, velocity - f])
                        .flatten();
                    
                    if cur_candidates.len() > 1 {
                        let mut new_candidates_lookup = new_candidates.collect::<HashSet<i64>>();
                        cur_candidates.retain(|c| new_candidates_lookup.contains(c));
                    } else {
                        cur_candidates.extend(new_candidates);
                    }
                }
                
                if all_found(&result_candidates) {
                    break 'outer;
                }
            }
        }
        
        if !all_found(&result_candidates) {
            panic!("Multiple candidates found, not supported yet! {:?}", &result_candidates);
        }

        let found_velocity = as_vector3(&result_candidates);
        println!("{:?}", found_velocity);

        let hailstone_a = self.hailstones[0].with_added_velocity(-found_velocity);
        let hailstone_b = self.hailstones[1].with_added_velocity(-found_velocity);
        
        // println!("Solving: {} and {}", hailstone_a, hailstone_b);
        
        // We could probably do this without floating points
        let intersection = hailstone_a.get_crossing(&hailstone_b)
            .expect("No intersection found for this velocity.");
        println!("Intersection found at: {}", intersection);

        let a_time = (intersection.x as i64 - hailstone_a.position.x) / hailstone_a.velocity.x;
        let a_time_1 = (intersection.y as i64 - hailstone_a.position.y) / hailstone_a.velocity.y;
        println!("{:?}", (a_time, a_time_1));
        let position = hailstone_a.position_at(a_time);
        
        Hailstone::new(position, found_velocity)
    }
}

fn all_found(candidates: &[HashSet<i64>]) -> bool {
    candidates.iter().all(|c| c.len() == 1)
}

fn as_vector3(candidates: &[HashSet<i64>; 3]) -> Vector3<i64> {
    Vector3::new(
        candidates[0].iter().next().unwrap().clone(),
        candidates[1].iter().next().unwrap().clone(),
        candidates[2].iter().next().unwrap().clone(),
    )
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
