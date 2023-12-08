use regex_macro::regex;
use std::{io::{stdin, BufRead}, ops::RangeInclusive};


fn main() {
    let races: Vec<_> = get_races(stdin().lock()).collect();
    // println!("{:#?}", &races);
    let mut product = 1;
    for race in races {
        let range = race.get_record_time_rage();
        let int_range = range.start().ceil() as isize..=range.end().floor() as isize;
        let span = (int_range.start() - int_range.end()).abs() + 1;
        println!("{:?} -> {:?} -> {:?} -> {}", &race, range, int_range, span);
        product *= span;
    }
    println!("{product}");
}

fn get_races(input: impl BufRead) -> impl Iterator<Item = Race> {

    let mut lines = input.lines();
    let time_numbers: Vec<_> = extract_numbers(&lines.next().unwrap().unwrap()).collect();
    let distance_numbers: Vec<_> = extract_numbers(&lines.next().unwrap().unwrap()).collect();

    time_numbers.into_iter().zip(distance_numbers).map(|(time, record)| Race { time, record })
}

fn extract_numbers(input: &str) -> impl Iterator<Item = usize> + '_ {
    let select_nums_regex = regex!(r".*:(.*)");
    let num_regex = regex!(r"\d+");
    
    let nums_str = select_nums_regex.captures(input).unwrap().get(1).unwrap().as_str();
    num_regex.find_iter(nums_str).map(|m| m.as_str().parse::<usize>().unwrap())
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
