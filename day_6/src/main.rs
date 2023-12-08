use regex_macro::regex;
use std::{io::{stdin, BufRead}, ops::RangeInclusive};


fn main() {
    let race = get_race(stdin().lock());

    let range = race.get_record_time_rage();
    let int_range = range.start().ceil() as isize..=range.end().floor() as isize;
    let span = (int_range.start() - int_range.end()).abs() + 1;
    println!("{:?} -> {:?} -> {:?} -> {}", &race, range, int_range, span);
}

fn get_race(input: impl BufRead) -> Race {
    let mut lines = input.lines();
    Race {
        time: extract_number(&lines.next().unwrap().unwrap()),
        record: extract_number(&lines.next().unwrap().unwrap()),
    }
}

fn extract_number(input: &str) -> usize {
    let select_nums_regex = regex!(r".*:(.*)");
    
    let nums_str = select_nums_regex.captures(input).unwrap().get(1).unwrap().as_str();
    remove_whitespaces(nums_str).parse::<usize>().unwrap()
}

// You can just remove the whitespaces from the input, but it took a minute to do this :)
fn remove_whitespaces(s: &str) -> String {
    s.split_whitespace().collect()
}

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn get_record_time_rage(&self) -> RangeInclusive<f64> {
        // Quadractic formula
        // a = -1
        // b = self.time
        // c = -self.record
        let plus_minus_part_squared = (self.time * self.time) - 4 * self.record;
        // Small time to beat the record
        let plus_minus_part = (plus_minus_part_squared as f64).sqrt() - 0.000001;
        let from = (-(self.time as f64) + plus_minus_part) / -2.0;
        let to = (-(self.time as f64) - plus_minus_part) / -2.0;
        from..=to
    }
}
