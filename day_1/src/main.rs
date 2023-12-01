use std::error::Error;
use std::io::{stdin, BufRead};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line_result in stdin().lock().lines() {
        let line = line_result?;
        let number = extract_number(&line);
        // println!("{} -> {}", line, number);
        sum += number;
    }
    println!("{}", sum);
    Ok(())
}

fn extract_number(s: &str) -> usize {
    let mut digits: Option<[char; 2]> = None;
    for ch in s.chars() {
        if !ch.is_numeric() {
            continue;
        }

        if let Some(existing_nums) = digits {
            digits = Some([existing_nums[0], ch]);
        } else {
            digits = Some([ch, ch]);
        }
    }

    if let Some(found_digits) = digits {
        return String::from_iter(found_digits).parse::<usize>().unwrap();
    } else {
        println!("Digits not found in string: {}", s);
        return 0;
    }
}
