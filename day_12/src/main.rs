use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let row = Row::from_str(&line).unwrap();
        println!("{:?}", &row);
    }
}

fn find_possible_combinations(tiles: &[Tile], known_sequence: &[usize]) -> usize {
    let known_damaged_count: usize = known_sequence.iter().sum();
    let revealed_damaged = tiles.iter().filter(|&t| t == &Tile::Damaged).count();
    let missing_damaged = known_damaged_count - revealed_damaged;
    println!("{} missing", missing_damaged);

    let unknown_indexes: Vec<_> = tiles
        .iter()
        .enumerate()
        .filter(|(_, &t)| t == Tile::Unknown)
        .map(|(i, _)| i)
        .collect();
    
    // TODO: Return case with one element in known_sequence
    let first_sequence_number = *known_sequence.first().unwrap();
    let mut temp_tiles = &mut tiles.iter().cloned().collect::<Vec<_>>()[..];
    if known_sequence.len() == 1 {
        for i in 0..1 {
            
        }
    }
    
    let first_unknown_index = *unknown_indexes.first().unwrap();
    /*
    for  in  {
        
    }
    */
    0
}

fn check_sequence(tiles: &[Tile], expected_sequence: &[usize]) -> bool {
    let mut next_expected = 0;
    let mut sequence_iter = expected_sequence.iter();
    for &tile in tiles.iter() {
        if next_expected == 0{
            if tile.is_damaged() {
                next_expected = match sequence_iter.next() {
                    Some(&n) => n,
                    None => return false,
                };
            }
            continue;
        }

        if next_expected == 1 {
            if tile.is_damaged() {
                return false;
            }
            next_expected -= 1;
            continue;
        }

        if !tile.is_damaged() {
            return false;
        }
        next_expected -= 1;
    }
    return next_expected <= 1 && sequence_iter.next().is_none();
}

fn all_numbers_are_consecutive(nums: &[usize]) -> bool {
    let mut iter = nums.iter();
    let mut cur = match iter.next() {
        Some(n) => *n,
        None => return false,
    };
    for &num in iter {
        if (cur + 1) != num {
            return false;
        }
        cur = num;
    }
    return true;
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
    known_sequence: Vec<usize>,
}

impl Row {
    fn find_possible_combinations(&self) -> usize {
        find_possible_combinations(&self.tiles, &self.known_sequence)
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        
        let tiles_str = split.next().expect("No tiles found in str.");
        let tiles: Vec<_> = tiles_str
            .chars()
            .map(Tile::try_from)
            .collect::<Result<_, _>>()
            .unwrap();

        let sequence_str = split.next().expect("No sequence found in str.");
        let expected_sequence: Vec<_> = sequence_str
            .split(',')
            .map(usize::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        
        Ok(Row { tiles, known_sequence: expected_sequence })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Operational,
    Damaged,
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
    use crate::all_numbers_are_consecutive;
    use crate::check_sequence;

    #[test]
    fn row_parses_correctly() {
        let row = Row::from_str("???.### 1,1,3").unwrap();
        let expected_tiles = [Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged];
        let expected_sequence = [1, 1, 3];

        assert_eq!(&row.tiles[..], &expected_tiles);
        assert_eq!(&row.known_sequence[..], &expected_sequence);
    }

    #[test]
    fn check_returns_true_1() {
        assert_check_returns("#.#.### 1,1,3", true)
    }

    #[test]
    fn check_returns_true_2() {
        assert_check_returns("..#.### 1,3", true)
    }

    #[test]
    fn check_returns_false_1() {
        assert_check_returns("???.### 1,1,3", false)
    }

    #[test]
    fn check_returns_false_2() {
        assert_check_returns("#.#..## 1,1,3", false)
    }

    #[test]
    fn check_returns_false_3() {
        assert_check_returns("#.#.##. 1,1,3", false)
    }

    fn assert_check_returns(s: &str, expected: bool) {
        let row = Row::from_str(s).unwrap();
        assert_eq!(check_sequence(&row.tiles, &row.known_sequence), expected);
    }

    #[test]
    fn consecutive_nums_return_true() {
        let nums = &[1, 2, 3, 4, 5];
        assert!(all_numbers_are_consecutive(nums))
    }

    #[test]
    fn consecutive_nums_return_false() {
        let nums = &[1, 2, 4, 5];
        assert!(!all_numbers_are_consecutive(nums))
    }

    /*
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
    */
}
