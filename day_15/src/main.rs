
fn main() {
    let path = std::env::current_dir().unwrap().join("day_15/input.txt");
    println!("Opening file: {}", path.display());
    println!();
    let content = std::fs::read_to_string(path).unwrap();

    part_1(&content);
}

fn part_1(input: &str) {
    let sum: usize = input.split(',').map(|s| holiday_hash(s) as usize).sum();
    println!("{sum}");
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
