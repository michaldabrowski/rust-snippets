use std::fs::read_to_string;

fn main() {
    let mut current_dial: u32 = 50;
    let mut password: u32 = 0;

    let input = read_to_string("input.txt").expect("Failed to read input file");

    for line in input.lines() {
        let num = line[1..].parse::<u32>().unwrap();
        let is_left = &line[0..1] == "L";

        password += count_zeros(current_dial, num, is_left);

        let num_mod = num % 100;
        if is_left {
            current_dial = (current_dial + 100 - num_mod) % 100;
        } else {
            current_dial = (current_dial + num_mod) % 100;
        }
    }
    println!("{}", password);
}

/// Count how many times we pass through 0 when moving `steps` from `position` in given direction
fn count_zeros(position: u32, steps: u32, is_left: bool) -> u32 {
    let first_zero = match (is_left, position) {
        (_, 0) => 100,
        (true, p) => p,
        (false, p) => 100 - p,
    };

    if steps >= first_zero {
        (steps - first_zero) / 100 + 1
    } else {
        0
    }
}
