use std::io::{stdin, BufRead};

fn main() {
    let mut sum = 0;
    for line_result in stdin().lines() {
        let line = line_result.unwrap();
        let nums: Vec<_> = line
            .split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        let next = predict_next_number(&nums);
        sum += next;
    }
    println!("{sum}");
}

fn predict_next_number(nums: &[isize]) -> isize {
    let mut next_iter = nums.into_iter();
    next_iter.next().expect("Empty slice.");
    
    
    let mut diffs = Vec::with_capacity(nums.len() - 1);
    let mut all_equal = true;
    let mut first_diff = None;
    for (&cur, &next) in nums.into_iter().zip(next_iter) {
        let diff = next - cur;
        diffs.push(diff);
        if first_diff.is_none() {
            first_diff = Some(diff);
            continue;
        }
        
        if diff != first_diff.unwrap() {
            all_equal = false;
        }
    }
    
    
    let next_expected_diff = if all_equal {
        first_diff.unwrap()
    } else {
        predict_next_number(&diffs)
    };
    
    // println!("{:?} -> {:?} = {}", &nums, &diffs, next_expected_diff);
    nums.last().unwrap() + next_expected_diff
}
