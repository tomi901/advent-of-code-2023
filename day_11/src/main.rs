use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::ops::Range;

fn main() {
    let universe = Universe::parse(stdin().lock());
    println!("Empty rows ({}): {:?}", universe.empty_rows.len(), universe.empty_rows);
    println!("Empty columns ({}): {:?}", universe.empty_rows.len(), universe.empty_columns);
    
    let pairs = universe.get_galaxy_pairs();
    // println!("Pairs ({}): {:?}", pairs.len(), &pairs);
    
    let expansion = 1000000;
    let sum: usize = pairs
        .iter()
        .map(|p| universe.get_distance(p, expansion))
        .sum();
    println!("Sum: {}", sum);
}

enum Tile {
    None,
    Galaxy,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point2D(usize, usize);

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

struct Universe {
    width: usize,
    height: usize,
    galaxies: HashSet<Point2D>,
    
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

impl Universe {
    fn parse(input: impl BufRead) -> Universe {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap().unwrap();
        
        let mut universe = Universe {
            width: first_line.len(),
            height: 0,
            galaxies: HashSet::new(),
            empty_rows: Vec::new(),
            empty_columns: Vec::new(),
        };
        universe.add_row_from_str(&first_line);

        for line_result in lines {
            let line = line_result.unwrap();
            universe.add_row_from_str(&line);
        }
        universe.recalculate_columns();
        universe
    }
    
    fn add_row_from_str(&mut self, s: &str) {
        let mut any_found = false;
        let y = self.height;
        for (x, ch) in s.chars().enumerate() {
            let point = Point2D::new(x, y);
            match ch {
                '.' => {},
                '#' => {
                    self.galaxies.insert(point);
                    any_found = true;
                },
                _ => panic!("Unexpected char: {}", ch),
            }
        }
        if !any_found {
            self.empty_rows.push(y);
        }
        self.height += 1;
    }
    
    fn recalculate_columns(&mut self) {
        self.empty_columns.clear();
        for x in 0..self.width {
            let any_found = (0..self.height).all(|y| {
                let point = Point2D::new(x, y);
                !self.galaxies.contains(&point)
            });
            if any_found {
                self.empty_columns.push(x);
            }
        }
    }
    
    fn get_galaxy_pairs(&self) -> Vec<GalaxyPair> {
        let mut pairs = vec![];
        let galaxies: Vec<_> = self.galaxies.iter().cloned().collect();
        for (i, &galaxy) in galaxies.iter().enumerate() {
            let pair_with = &galaxies[(i + 1)..];
            for &other in pair_with.iter() {
                pairs.push(GalaxyPair(galaxy, other));
            }
        }
        pairs
    }
    
    fn get_distance(&self, pair: &GalaxyPair, expansion: usize) -> usize {
        let a = pair.0;
        let b = pair.1;
        let x_distance = a.0.abs_diff(b.0);
        let y_distance = a.1.abs_diff(b.1);
        let raw_distance = x_distance + y_distance;

        // println!();
        let empty_columns = self.get_empty_columns(a.0..b.0).len();
        let empty_rows = self.get_empty_rows(a.1..b.1).len();

        let distance = raw_distance + ((empty_rows + empty_columns) * (expansion - 1));
        // println!("Distance from {:?} to {:?} = {} {:?}", &a, &b, raw_distance, &(x_distance, y_distance));
        // println!("+ ({} empty rows + {} empty_columns)", empty_rows, empty_columns);
        // println!("= * {}", expansion - 1);
        // println!("= {}", distance);

        distance
    }
    
    fn get_empty_columns(&self, range: Range<usize>) -> &[usize] {
        Self::get_range(&self.empty_columns, range)
    }
    
    fn get_empty_rows(&self, range: Range<usize>) -> &[usize] {
        Self::get_range(&self.empty_rows, range)
    }
    
    fn get_range(nums: &[usize], range: Range<usize>) -> &[usize] {
        let actual_range = if range.start <= range.end {
            range
        } else {
            range.end..range.start
        };
        let from = match nums.binary_search(&actual_range.start) {
            Ok(i) => i,
            Err(i) => i,
        };
        let to = match nums.binary_search(&actual_range.end) {
            Ok(i) => i,
            Err(i) => i,
        };
        let result = &nums[from..to];
        // println!("{:?} {:?} -> {:?} ({:?})", actual_range, nums, &result, from..to);
        result
    }
}

#[derive(Debug)]
struct GalaxyPair(Point2D, Point2D);
