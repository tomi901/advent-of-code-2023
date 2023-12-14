use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::{BufRead, stdin};
use std::ops::Range;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    
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

    /*
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
    */
}

// Answer copied from https://pastebin.com/1EAdWWMk
// I tried to learn from it at the very least
fn find_possible_combinations_cached(
    cache: &mut HashMap<(usize, usize, u16), usize>,
    tiles: &[Tile],
    groups: &[u16],
    from_i: usize,
    group_i: usize,
    size: u16,
) -> usize {
    if from_i >= tiles.len() {
        // Exhausted all groups
        if group_i >= groups.len() {
            return 1;
        }
        
        // The line ends with a "damaged" symbol and we've matched that last group
        if group_i >= groups.len() - 1 && groups[group_i] == size {
            return 1;
        }
        
        return 0;
    }

    return match tiles[from_i] {
        Tile::Operational => {
            // Skip sequence of operational spots
            if size == 0 {
                return find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i, size);
            }

            // The current combination failed to match a proper sequence from the input
            if group_i >= groups.len() || size != groups[group_i] {
                return 0;
            }

            // we have a match: process the next group
            find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i + 1, 0)
        },
        Tile::Damaged => {
            // We don't expect any other damaged spots, failed to match
            if group_i >= groups.len() || size + 1 > groups[group_i] {
                return 0;
            }

            find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i, size + 1)
        },
        Tile::Unknown => {
            if let Some(&answer) = cache.get(&(from_i, group_i, size)) {
                return answer;
            }
            
            let mut combinations = 0;

            // if we did not encounter any damaged cells,
            // we can treat this one as undamaged
            if size == 0 {
                combinations += find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i, size);
            }

            // If we need more damaged cells to complete our match,
            // we can treat the current cell as damaged
            if group_i < groups.len() && size < groups[group_i] {
                combinations += find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i, size + 1);
            }

            // We have the correct number of damaged cells, so we can just
            // treat this one as undamaged in order to complete the match
            if group_i < groups.len() && size == groups[group_i] {
                combinations += find_possible_combinations_cached(cache, tiles, groups, from_i + 1, group_i + 1, 0);
            }

            cache.insert((from_i, group_i, size), combinations);
            combinations
        },
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
    known_sequence: Vec<u16>,
}

impl Row {
    fn find_possible_combinations(&self) -> usize {
        let mut cache = HashMap::default();
        find_possible_combinations_cached(&mut cache, &self.tiles, &self.known_sequence, 0, 0, 0)
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
            .map(u16::from_str)
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
