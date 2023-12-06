use std::{io::{stdin, Lines, BufRead, StdinLock}, cmp::{max, min}, collections::{HashMap, hash_map::Entry}};

struct CharMap {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct GearRatio {
    total: usize,
    count: usize,
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

    pub fn check_gear_positions(&self, from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
        let from_clamped = (max(from.0, 0), max(from.1, 0));
        let to_clamped = (min(to.0, self.width), min(to.1, self.height));

        let mut gears_positions = Vec::new();
        for x in (from_clamped.0)..=(to_clamped.0) {
            for y in (from_clamped.1)..=(to_clamped.1) {
                let ch = self.get_char(x, y);
                if ch == '*' {
                    gears_positions.push((x, y));
                }
            }
        }
        gears_positions
    }
}

fn main() {
    let map = CharMap::from_lines(&mut stdin().lock().lines());
    let mut cur_number_str = String::default();
    let mut cur_range = 0..0;

    let mut gear_ratios: HashMap<(usize, usize), GearRatio> = HashMap::new();

    for (line_index, line) in map.lines().enumerate() {
        for (ch_index, ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                if cur_number_str.is_empty() {
                    cur_range = ch_index..(ch_index + 1)
                } else {
                    cur_range = cur_range.start..(ch_index + 1)
                }
                cur_number_str.push(*ch);
            }
            
            if !cur_number_str.is_empty() && (!ch.is_numeric() || (ch_index + 1) >= line.len()) {
                let from = (
                    if cur_range.start > 0 { cur_range.start - 1 } else { 0 },
                    if line_index > 0 { line_index - 1 } else { 0 });
                let to = (cur_range.end, line_index + 1);

                let gear_positions = map.check_gear_positions(from, to);
                let num = cur_number_str.parse::<usize>().unwrap();
                cur_number_str.clear();

                for pos in &gear_positions {
                    println!("{line_index} {ch_index} {} {pos:?} -> {num}", &gear_positions.len());
                    match gear_ratios.entry(*pos) {
                        Entry::Occupied(o) => {
                            let entry = o.into_mut();
                            entry.count += 1;
                            entry.total *= num;
                        },
                        Entry::Vacant(v) => {
                            v.insert(GearRatio { total: num, count: 1 });
                        },
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for ratio in gear_ratios.iter().filter(|kvp| kvp.1.count > 1) {
        println!("{ratio:?}");
        sum += ratio.1.total;
    }

    // println!("{:?}", gear_ratios);
    println!("{}", sum);
}
