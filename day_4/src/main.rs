use std::{io::{stdin, BufRead}, collections::{HashSet, hash_map::RandomState}};
use regex_macro::regex;

fn main() {
    let card_regex = regex!(r"(?i)card\s+(\d*): (.+) \| (.+)");

    let mut cards: Vec<CardStack> = Vec::new();
    for line_result in stdin().lock().lines() {
        let line = line_result.expect("Line error");
        let captures = card_regex.captures(&line).expect("Captures error");
        
        let matches_count = get_points(
            captures.get(2).expect("Capture 2 failed").as_str(),
            captures.get(3).expect("Capture 3 failed").as_str(),
        );
        cards.push(CardStack { matches_count, count: 1 });
    }
    
    let mut sum = 0;
    for card_index in 0..cards.len() {
        let next_index = card_index + 1;
        let card = &cards[card_index];
        let count = card.count;
        println!("Card {} ({} matches):", card_index + 1, card.matches_count);
        sum += card.total_points();
        for change_index in next_index..(next_index + card.matches_count) {
            cards[change_index].add(count);
            println!("- Adding to card {}", change_index + 1);
        }
    }

    // println!("{:#?}", cards);
    println!("{sum}");
    println!("{}", cards.iter().map(|c| c.count).sum::<usize>());
}

fn get_points(winner_nums: &str, nums: &str) -> usize {
    let winners: HashSet<i32, RandomState> = HashSet::from_iter(get_nums(winner_nums));
    get_nums(nums).filter(|n| winners.contains(n)).count()
}

fn get_nums(nums: &str) -> impl Iterator<Item = i32> + '_ {
    let separator = regex!(r"\d+");
    separator.find_iter(&nums).map(|s| s.as_str().parse::<i32>().expect("Parse fail"))
}

#[derive(Debug)]
struct CardStack {
    matches_count: usize,
    count: usize,
}

impl CardStack {
    pub fn add(&mut self, amount: usize) {
        self.count += amount;
    }

    pub fn total_points(&self) -> usize {
        if self.matches_count > 0 {
            2_usize.pow((self.matches_count - 1_usize).try_into().unwrap()) * self.count
        } else {
            0
        }
    }
}
