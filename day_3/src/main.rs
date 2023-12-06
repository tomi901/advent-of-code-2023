use std::{io::{stdin, Lines, BufRead, StdinLock}, cmp::{max, min}};

struct CharMap {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl CharMap {
    pub fn from_lines(lines: &mut Lines<StdinLock>) -> CharMap {
        let first = lines.next().unwrap().unwrap();
        let mut chars: Vec<char> = first.chars().collect();

        let width = chars.len();
        let mut height = 0;

        for line in lines {
            chars.extend(line.unwrap().chars());
            height += 1;
        }
        CharMap { chars, width, height }
    }

    pub fn lines(&self) -> impl Iterator<Item = &[char]> {
        (0..=self.height).into_iter().map(|y| {
            let start_index = y * self.width;
            &self.chars[start_index..start_index + self.width]
        })
    }

    pub fn get_char(&self, x: usize, y: usize) -> char {
        self.chars[x + (y * self.width)]
    }

    pub fn check_special_chars(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from_clamped = (max(from.0, 0), max(from.1, 0));
        let to_clamped = (min(to.0, self.width), min(to.1, self.height));

        for x in (from_clamped.0)..=(to_clamped.0) {
            for y in (from_clamped.1)..=(to_clamped.1) {
                let ch = self.get_char(x, y);
                if ch != '.' && !ch.is_numeric() {
                    // println!("Found {ch}");
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let map = CharMap::from_lines(&mut stdin().lock().lines());
    let mut cur_number_str = String::default();
    let mut cur_range = 0..0;

    let mut sum = 0;
    for (line_index, line) in map.lines().enumerate() {
        for (ch_index, ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                if cur_number_str.is_empty() {
                    cur_range = ch_index..(ch_index + 1)
                } else {
                    cur_range = cur_range.start..(ch_index + 1)
                }
                cur_number_str.push(*ch);
            } else if !cur_number_str.is_empty() {
                let from = (
                    if cur_range.start > 0 { cur_range.start - 1 } else { 0 },
                    if line_index > 0 { line_index - 1 } else { 0 });
                let to = (cur_range.end, line_index + 1);
                if map.check_special_chars(from, to) {
                    println!("{}: ({from:?}/{to:?}) => {cur_number_str}", line_index + 1);
                    sum += cur_number_str.parse::<usize>().unwrap();
                }
                cur_number_str.clear();
            }
        }

        let from = (
            if cur_range.start > 0 { cur_range.start - 1 } else { 0 },
            if line_index > 0 { line_index - 1 } else { 0 });
        let to = (cur_range.end, line_index + 1);
        if !cur_number_str.is_empty() && map.check_special_chars(from, to) {
            println!("{}: ({from:?}/{to:?}) => {cur_number_str}", line_index + 1);
            sum += cur_number_str.parse::<usize>().unwrap();
        }
        cur_number_str.clear();
    }

    println!("{}", sum);
}
