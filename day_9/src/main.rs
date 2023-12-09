use std::io::stdin;

fn main() {
    let mut previous_sum = 0;
    let mut next_sum = 0;
    for line_result in stdin().lines() {
        let line = line_result.unwrap();
        let nums: Vec<_> = line
            .split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        previous_sum += predict_previous_number(&nums);
        next_sum += predict_next_number(&nums);
    }
    println!("Previous sum: {previous_sum}");
    println!("Next sum: {next_sum}");
}

fn predict_next_number(nums: &[isize]) -> isize {
    let diff_result = get_diffs(&nums);
    
    let next_expected_diff = if diff_result.all_equal {
        *diff_result.diffs.first().unwrap()
    } else {
        predict_next_number(&diff_result.diffs)
    };
    
    // println!("{:?} -> {:?} = {}", &nums, &diffs, next_expected_diff);
    nums.last().unwrap() + next_expected_diff
}

fn predict_previous_number(nums: &[isize]) -> isize {
    let diff_result = get_diffs(&nums);

    let next_expected_diff = if diff_result.all_equal {
        *diff_result.diffs.first().unwrap()
    } else {
        predict_previous_number(&diff_result.diffs)
    };

    // println!("{:?} -> {:?} = {}", &nums, &diffs, next_expected_diff);
    nums.first().unwrap() - next_expected_diff
}

fn get_diffs(nums: &[isize]) -> DiffResult {
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
    
    DiffResult { diffs, all_equal }
}

struct DiffResult {
    diffs: Vec<isize>,
    all_equal: bool,
}
