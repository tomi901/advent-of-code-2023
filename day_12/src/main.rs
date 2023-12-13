use std::cmp::min;
use std::fmt::Write;
use std::io::{BufRead, stdin};
use std::ops::Range;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    
    /*
    let mut sum = 0;
    for (i, line_result) in stdin().lock().lines().enumerate() {
        let local_now = Instant::now();
        let line = line_result.unwrap();
        let row = Row::from_str(&line).unwrap();
        // println!("{:?}", &row);
        let result = row.find_possible_combinations();
        sum += result;
        println!("{}) [{} element/s] -> {}", i + 1, row.tiles.len(), result);
        println!(" - Elapsed: {:?}", local_now.elapsed());
    }
    println!("Total elapsed: {:?}", now.elapsed());
    println!("{sum}");
    */

    let rows: Vec<_> = stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(i, s)| {
            (i + 1, Row::from_str(&s.unwrap()).unwrap())
        })
        .collect();

    let progress = Arc::new(Mutex::new(0));
    let total_length = rows.len();
    let sum: usize = rows
        .into_par_iter()
        .map(move |(i, row)| {
            let progress_ref = progress.clone();
            let local_now = Instant::now();
            let result = row.find_possible_combinations();

            let current_progress = {
                let mut mutex = progress_ref.lock().unwrap();
                let current_progress = *mutex + 1;
                *mutex = current_progress;
                current_progress
            };
            let current_time = now.elapsed();
            println!("({}/{}) N°{} finished. Current elapsed: {:?}, Total elapsed: {:?}",
                current_progress, total_length, i + 1, local_now.elapsed(), current_time);
            if current_progress % 5 == 0 {
                let average_time = current_time / current_progress;
                let estimated_time = average_time * total_length as u32;
                let eta = estimated_time - current_time;
                println!("ETA: {eta:?}, Average time: {average_time:?}, Total estimated: {estimated_time:?}");
            }
            result
        })
        .sum();

    println!("Total elapsed: {:?}", now.elapsed());
    println!("{sum}");
}

fn find_possible_combinations(current_group: usize, tiles: &[Tile], next_groups: &[usize]) -> usize {
    if tiles.len() == 0 {
        return 0;
    }
    // println!("Testing ({:?}, {:?}) on {:?}", current_group, &next_groups, &tiles);
    if next_groups.len() == 0 {
        let limit = tiles.len() - current_group + 1;
        // println!("- Will test {} combination/s", {limit});
        let combinations = (0..limit)
            .filter(|&i| {
                let check_range = i..(i + current_group);
                can_place_group(&check_range, &tiles)
            })
            .count();
        // println!("- Found {} combination/s", combinations);
        return combinations;
    }
    
    let min_length = current_group + next_groups.iter().sum::<usize>() + next_groups.len();
    let first_damaged_pos = tiles.iter().position(Tile::is_damaged);
    let limit = tiles.len() - min_length;
    let actual_limit = min(limit, first_damaged_pos.unwrap_or(usize::MAX)) + 1;
    // println!("- Will recursively test {} combination/s (Min length {}/{})", actual_limit, min_length, tiles.len());

    let next_group = *next_groups.first().unwrap();
    let combinations = (0..actual_limit)
        .map(|i| {
            let check_range = i..(i + current_group);
            if can_place_group(&check_range, tiles) {
                let next_rest = &tiles[(check_range.end + 1)..];
                find_possible_combinations(next_group, next_rest, &next_groups[1..])
            } else {
                0
            }
        })
        .sum();
    // println!("- Summed to {} combination/s", combinations);
    combinations
}

fn can_place_group(group: &Range<usize>, tiles: &[Tile]) -> bool {
    let tiles_slice = &tiles[group.clone()];
    if tiles_slice.len() != group.len() {
        return false;
    }
    if tiles_slice.iter().any(|&t| t == Tile::Operational) {
        return false;
    }
    match tiles.get(group.end) {
        Some(&next_tile) if next_tile == Tile::Damaged => false,
        _ => true,
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
    known_sequence: Vec<usize>,
}

impl Row {
    fn find_possible_combinations(&self) -> usize {
        let first_group = *self.known_sequence.first().unwrap();
        let rest = &self.known_sequence[1..];
        find_possible_combinations(first_group, &self.tiles, rest)
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXTEND: usize = 5;

        let mut split = s.split(' ');
        
        let tiles_str = split.next().expect("No tiles found in str.");
        let temp_tiles: Vec<_> = tiles_str
            .chars()
            .map(Tile::try_from)
            .collect::<Result<_, _>>()
            .unwrap();
        let mut tiles = temp_tiles.clone();
        for _ in 0..(EXTEND - 1) {
            tiles.push(Tile::Unknown);
            tiles.extend(temp_tiles.iter());
        }

        let sequence_str = split.next().expect("No sequence found in str.");
        let temp_known_sequence: Vec<_> = sequence_str
            .split(',')
            .map(usize::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        let mut known_sequence = temp_known_sequence.clone();
        for _ in 0..(EXTEND - 1) {
            known_sequence.extend(temp_known_sequence.iter());
        }
        
        Ok(Row { tiles, known_sequence })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Operational,
    Damaged,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Unknown => '?',
            Tile::Operational => '.',
            Tile::Damaged => '#',
        };
        f.write_char(ch)
    }
}

impl Tile {
    fn is_damaged(&self) -> bool {
        *self == Tile::Damaged
    }
}

#[derive(Debug)]
pub struct InvalidCharError {
    pub c: char,
}

impl TryFrom<char> for Tile {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(InvalidCharError { c: value }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Row;
    use crate::Tile::*;

    #[test]
    fn row_parses_correctly() {
        let row = Row::from_str("???.### 1,1,3").unwrap();
        let expected_tiles = [Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged];
        let expected_sequence = [1, 1, 3];

        assert_eq!(&row.tiles[..], &expected_tiles);
        assert_eq!(&row.known_sequence[..], &expected_sequence);
    }

    mod examples {
        use std::str::FromStr;
        use crate::Row;

        fn assert_combinations(s: &str, expected: usize) {
            let row = Row::from_str(s).unwrap();
            let result = row.find_possible_combinations();
            assert_eq!(result, expected);
        }

        #[test]
        fn returns_correct_combinations_case_1() {
            assert_combinations("???.### 1,1,3", 1);
        }

        #[test]
        fn returns_correct_combinations_case_2() {
            assert_combinations(".??..??...?##. 1,1,3", 4);
        }

        #[test]
        fn returns_correct_combinations_case_3() {
            assert_combinations("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        }

        #[test]
        fn returns_correct_combinations_case_4() {
            assert_combinations("????.#...#... 4,1,1", 1);
        }

        #[test]
        fn returns_correct_combinations_case_5() {
            assert_combinations("????.######..#####. 1,6,5", 4);
        }

        #[test]
        fn returns_correct_combinations_case_6() {
            assert_combinations("?###???????? 3,2,1", 10);
        }
    }
}
