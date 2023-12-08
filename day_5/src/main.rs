use std::ops::{Range, RangeInclusive};
use std::io::{stdin, BufRead};
use rayon::prelude::*;

fn main() {
    println!("Determining seeds");
    let seeds = get_seeds(stdin().lock());
    println!("Using seeds: {:?}", seeds);

    let maps = create_maps(&mut stdin().lock());
    // println!("Maps: {:#?}", maps);

    let candidates = get_candidates(&seeds, &maps);
    // println!("Candidates ({}): {:?}", candidates.len(), candidates);

    let result = get_lowest_value(&candidates, &maps);
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

fn get_candidates(seeds: &Vec<Range<isize>>, transform_maps: &Vec<TransformMap>) -> Vec<isize> {
    let mut candidates: Vec<_> = seeds.iter().map(|s| s.start).collect();
    for (i, map) in transform_maps.iter().enumerate() {
        let cur_candidates = get_candidates_from_map(map, &transform_maps[..i])
            .filter(|c| seeds.iter().any(|s| s.contains(c)));
        candidates.extend(cur_candidates)
    }
    candidates
}

fn get_candidates_from_map<'a>(map: &'a TransformMap, previous: &'a [TransformMap]) -> impl Iterator<Item = isize> + 'a {
    map.0.iter().map(|m| {
        previous.into_iter().rev().try_fold(m.source_start, |n, m| {
            m.try_inverse_transform(n)
        })
    }).flatten()
}

fn get_lowest_value(seeds: &Vec<isize>, transform_maps: &Vec<TransformMap>) -> Option<isize> {
    seeds.into_iter()
        .map(|&seed| process_value(seed, transform_maps))
        .min()
}

fn get_lowest_value_from_range(seeds: &Vec<Range<isize>>, transform_maps: &Vec<TransformMap>) -> Option<isize> {
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

    pub fn try_inverse_transform(&self, n: isize) -> Option<isize> {
        self.0.iter().map(|r| r.try_inverse_transform(n)).flatten().next()
    }
}

#[derive(Debug)]
pub struct MapRange {
    destination_start: isize,
    source_start: isize,
    length: isize,
}

impl MapRange {
    pub fn new(destination_start: isize, source_start: isize, length: isize) -> MapRange {
        MapRange { destination_start, source_start, length }
    }

    pub fn source_range(&self) -> Range<isize> {
        self.source_start..(self.source_start + self.length)
    }

    pub fn destination_range(&self) -> Range<isize> {
        self.destination_start..(self.destination_start + self.length)
    }

    pub fn try_transform(&self, n: isize) -> Option<isize> {
        Self::try_transform_from_range(self.source_range(), self.destination_start, n)
    }

    pub fn try_inverse_transform(&self, n: isize) -> Option<isize> {
        Self::try_transform_from_range(self.destination_range(), self.source_start, n)
    }

    fn try_transform_from_range(from: Range<isize>, to: isize, n: isize) -> Option<isize> {
        if from.contains(&n) {
            let relative_value = n - from.start;
            Some(to + relative_value)
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
