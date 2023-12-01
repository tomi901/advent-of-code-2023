use std::error::Error;
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Digits(usize, usize);

impl Digits {
    fn append_digits(digits: Option<Digits>, digit: usize) -> Option<Digits> {
        if let Some(existing_digits) = digits {
            Some(Digits(existing_digits.0, digit))
        } else {
            Some(Digits(digit, digit))
        }
    }

    fn to_number(&self) -> usize {
        (self.0 * 10 + self.1).into()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line_result in stdin().lock().lines() {
        let line = line_result?;
        let digits = extract_digits(&line);
        
        if let Some(found_digits) = digits {
            let number = found_digits.to_number();
            // println!("{} -> {:?} -> {}", line, found_digits, number);
            sum += number;
        } else {
            println!("No digits found in line: {}", line);
        }
    }
    println!("{}", sum);
    Ok(())
}

fn extract_digits(s: &str) -> Option<Digits> {
    let mut digits: Option<Digits> = None;
    for (i, ch) in s.chars().enumerate() {
        if ch.is_numeric() {
            let digit = ch as usize - '0' as usize;
            digits = Digits::append_digits(digits, digit);
        } else if let Some(digit) = try_get_number_name(&s[i..]) {
            digits = Digits::append_digits(digits, digit)
        }
    }
    digits
}

fn try_get_number_name(s: &str) -> Option<usize> {
    const NUMBER_NAMES: [&str; 10] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    for (i, name) in NUMBER_NAMES.iter().enumerate() {
        if s.starts_with(name) {
            return Some(i);
        }
    }
    return None;
}
