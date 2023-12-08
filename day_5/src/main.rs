use std::ops::Range;
use std::io::{stdin, BufRead};
use rayon::prelude::*;

fn main() {
    println!("Determining seeds");
    let seeds = get_seeds(stdin().lock());
    println!("Using seeds: {:?}", seeds);

    let maps = create_maps(&mut stdin().lock());
    // println!("Maps: {:#?}", maps);

    let result = get_lowest_value(&seeds, &maps);
    println!("Result: {:?}", result);
}

fn get_seeds(input: impl BufRead) -> Vec<Range<isize>> {
    let line = input.lines().next().expect("No seeds found.").expect("Error reading seeds.");
    let seeds_str = line.trim_start_matches("seeds: ");
    let mut seed_values_iter = seeds_str
        .split(' ')
        .map(|x| x.parse::<isize>().expect("Error parsing seed value."));
    let mut seeds = vec![];
    while let Some(num_start) = seed_values_iter.next() {
        let length = seed_values_iter.next().expect("No length defined for range");
        let range = num_start..(num_start + length);
        // println!("Adding seed range {:?}", range);
        seeds.push(range);
    }
    seeds
}

fn create_maps(input: &mut impl BufRead) -> Vec<TransformMap> {
    let mut maps = vec![];
    while let Some(line_result) = input.lines().next() {
        let line = line_result.expect("Error reading line");
        if line.ends_with("map:") {
            maps.push(create_map(input));
        }
    }
    maps
}

fn create_map(input: &mut impl BufRead) -> TransformMap {
    let mut ranges = vec![];
    for line_result in input.lines() {
        let line = line_result.expect("Couldn't read ranges line");
        if line.is_empty() {
            break;
        }

        let mut values = line.split(' ');
        let destination = values.next().expect("No destination defined").parse::<isize>().unwrap();
        let source = values.next().expect("No source defined").parse::<isize>().unwrap();
        let range = values.next().expect("No range defined").parse::<isize>().unwrap();
        ranges.push(MapRange::new(destination, source, range))
    }
    TransformMap::new(ranges)
}

fn get_lowest_value(seeds: &Vec<Range<isize>>, transform_maps: &Vec<TransformMap>) -> Option<isize> {
    seeds.into_par_iter()
        .enumerate()
        .map(|(i, seeds)| get_lowest_range_value(i, seeds.clone(), &transform_maps).unwrap())
        .min()
}

fn get_lowest_range_value(index: usize, seeds: Range<isize>, transform_maps: &Vec<TransformMap>) -> Option<isize> {
    println!("Processing range ({}) {:?}...", index, &seeds);
    let result = seeds.clone().into_par_iter()
        .map(|s| process_value(s, &transform_maps))
        .min();
    println!("Finised ({}) {:?}! result = {:?}", index, &seeds, result);
    result
}

fn process_value(seed: isize, transform_maps: &Vec<TransformMap>) -> isize {
    transform_maps.iter().fold(seed, |s, t| t.transform(s))
}

#[derive(Debug)]
pub struct TransformMap(Vec<MapRange>);

impl TransformMap {
    pub fn new(vec: Vec<MapRange>) -> TransformMap {
        TransformMap(vec)
    }

    pub fn transform_many(&self, nums: &mut [isize]) {
        for n in nums.as_mut() {
            *n = self.transform(*n);
        }
    }

    pub fn transform(&self, n: isize) -> isize {
        self.try_transform(n).unwrap_or(n)
    }

    pub fn try_transform(&self, n: isize) -> Option<isize> {
        self.0.iter().map(|r| r.try_transform(n)).flatten().next()
    }
}

#[derive(Debug)]
pub struct MapRange {
    source_range: Range<isize>,
    destination: isize,
}

impl MapRange {
    pub fn new(destination_start: isize, source_start: isize, length: isize) -> MapRange {
        MapRange { source_range: source_start..(source_start + length), destination: destination_start }
    }

    pub fn try_transform(&self, n: isize) -> Option<isize> {
        if self.source_range.contains(&n) {
            let relative_value = n - self.source_range.start;
            Some(self.destination + relative_value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MapRange, TransformMap};

    #[test]
    fn transforms_correctly_inside_range() {
        let range = MapRange::new(52, 50, 48);

        let result = range.try_transform(67);

        assert_eq!(result, Some(69));
    }

    #[test]
    fn transforms_correctly_outside_range() {
        let range = MapRange::new(52, 50, 48);

        let result = range.try_transform(42);

        assert_eq!(result, None);
    }

    #[test]
    fn map_transforms_correctly_inside_range() {
        let map = TransformMap::new(vec![
            MapRange::new(50, 98, 2),
            MapRange::new(52, 50, 48),
        ]);

        let result = map.transform(99);

        assert_eq!(result, 51);
    }

    #[test]
    fn map_transforms_correctly_outside_range() {
        let map = TransformMap::new(vec![
            MapRange::new(50, 98, 2),
            MapRange::new(52, 50, 48),
        ]);

        let result = map.transform(42);

        assert_eq!(result, 42);
    }
}
