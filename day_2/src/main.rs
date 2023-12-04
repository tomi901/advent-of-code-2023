use std::cmp::max;
use std::error::Error;
use std::io::{stdin, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;
use regex_macro::regex;

#[derive(Default, Debug)]
struct Cubes {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

#[derive(Debug)]
enum ParseCubesError {
    RegexFailed,
    NumberNotFound,
    NumberNotParseable(ParseIntError),
    ColorNotFound,
    InvalidColor(String),
}

impl Cubes {
    pub fn set_color(&mut self, color: &str, amount: usize) -> Result<(), ParseCubesError> {
        match color {
            "red" => self.red = amount,
            "blue" => self.blue = amount,
            "green" => self.green = amount,
            _ => return Err(ParseCubesError::InvalidColor(color.to_string())),
        };
        Ok(())
    }

    pub fn is_superset_of(&self, other: &Cubes) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn get_minimum_possible(&self, other: &Cubes) -> Cubes {
        Cubes {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    pub fn product(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Cubes {
    type Err = ParseCubesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();
        let combination_info_regex = regex!(r"(?i)[.\s]*([0-9]*) (\w*)");

        for combination in s.split(',') {
            let parse_result = combination_info_regex.captures(combination).ok_or(ParseCubesError::RegexFailed)?;
            let amount = parse_result
                .get(1)
                .ok_or(ParseCubesError::NumberNotFound)?
                .as_str()
                .parse::<usize>()
                .map_err(|err| ParseCubesError::NumberNotParseable(err))?;

            let color = parse_result
                .get(2)
                .ok_or(ParseCubesError::ColorNotFound)?
                .as_str();

            cubes.set_color(color, amount)?;
        }

        Ok(cubes)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let game_regex = regex!(r"(?i)game (\d*):");

    let mut sum = 0;
    for line_result in stdin().lock().lines() {
        let line = line_result?;
        let game_id_match = game_regex
            .find(&line)
            .expect("No match found.");

        let game_results = &line[game_id_match.range().end..];
        let minimum_cubes = game_results
            .split(';')
            .map(|r| Cubes::from_str(r))
            .flatten()
            .reduce(|lhs, rhs| lhs.get_minimum_possible(&rhs))
            .unwrap_or_default();

        sum += minimum_cubes.product();
    }

    println!("{sum}");
    Ok(())
}
