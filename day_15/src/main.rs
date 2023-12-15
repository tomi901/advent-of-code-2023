use std::{fmt::Debug, ops::Deref};
use array_init::array_init;

fn main() {
    let path = std::env::current_dir().unwrap().join("day_15/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let content = std::fs::read_to_string(path).unwrap();

    // part_1(&content);
    part_2(&content);
}

fn part_1(input: &str) {
    let sum: usize = input.split(',').map(|s| holiday_hash(s) as usize).sum();
    println!("{sum}");
}

fn part_2(input: &str) {
    let mut hashmap = HolidayHashMap::default();
    for instruction in input.split(',') {
        if instruction.ends_with('-') {
            let label = instruction.trim_end_matches('-');
            hashmap.remove_lens(label);
        } else if let Some((label, focal_length)) = try_get_assign_instruction(instruction) {
            hashmap.insert_lens(label, focal_length);
        } else {
            panic!("{}", instruction);
        }
    }
    println!("{:?}", &hashmap);
    println!("{}", hashmap.calculate_focusing_power());
    
}

fn try_get_assign_instruction(s: &str) -> Option<(&str, u8)> {
    let mut split = s.split('=');
    split.next()
        .map(|label| split
            .next()
            .map(|n| (label, n.parse::<u8>().expect("Focal length needs to be u8"))))
        .flatten()
}

fn holiday_hash(s: &str) -> u8 {
    let mut value: u8 = 0;
    for byte in s.bytes() {
        value = value.wrapping_add(byte);
        value = value.wrapping_mul(17);
    }
    // println!("{value}");
    value
}

struct HolidayHashMap([Option<Vec<Lens>>; 256]);

impl HolidayHashMap {
    pub fn insert_lens(&mut self, label: &str, focal_length: u8) {
        let position = holiday_hash(label) as usize;
        if let Some(some_box) = self.0[position].as_mut() {
            Self::insert_lens_in_vec(some_box, label, focal_length);
        } else {
            self.0[position] = Some(vec![Lens::new(label, focal_length)])
        }
    }

    // We could have a LensBox struct that handles this
    fn insert_lens_in_vec(vec: &mut Vec<Lens>, label: &str, focal_length: u8) {
        let lens_to_replace = vec.iter_mut().find(|lens| lens.has_label(label));
        if let Some(mut lens) = lens_to_replace {
            lens.focal_length = focal_length;
        } else {
            vec.push(Lens::new(label, focal_length))
        }
    }

    pub fn remove_lens(&mut self, label: &str) -> bool {
        let position = holiday_hash(label) as usize;
        if let Some(some_box) = self.0[position].as_mut() {
            if let Some(pos) = some_box.iter().position(|l| l.has_label(label)) {
                some_box.remove(pos);
                return true;
            }
        }
        return false;
    }

    pub fn calculate_focusing_power(&self) -> usize {
        let mut sum = 0;
        for (box_i, cur_box) in self.0.iter().enumerate() {
            if let Some(b) = cur_box {
                for (lens_i, lens) in b.iter().enumerate() {
                    let power = (box_i + 1) * (lens_i + 1) * lens.focal_length as usize;
                    sum += power;
                }
            }
        }
        sum
    }
}

impl Default for HolidayHashMap {
    fn default() -> Self {
        Self(array_init(|_| None))
    }
}

impl Debug for HolidayHashMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cur_box) in self.0.iter().enumerate() {
            match cur_box {
                Some(b) if b.len() > 0 => writeln!(f, "Box {}: {:?}", i, b)?,
                _ => {},
            }
        }
        Ok(())
    }
}

struct Lens {
    label: String, // This can be transformed into a &'a, not going to complicate the code like this
    focal_length: u8,
}

impl Lens {
    fn new(label: &str, focal_length: u8) -> Self {
        Self { label: label.to_owned(), focal_length }
    }

    fn has_label(&self, label: &str) -> bool {
        self.label == label
    }
}

impl Debug for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", &self.label, self.focal_length)
    }
}
