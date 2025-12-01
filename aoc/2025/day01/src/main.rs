use std::fs::read_to_string;

fn main() {
    let mut current_dial: u32 = 50;
    let mut password1a: u32 = 0;
    let mut password1b: u32 = 0;

    let input = read_to_string("input.txt").expect("Failed to read input file");

    for line in input.lines() {
        let num = line[1..].parse::<u32>().unwrap();
        let is_left = &line[0..1] == "L";

        password1b += count_zeros_on_the_way(current_dial, num, is_left);

        let num_mod = num % 100;
        if is_left {
            current_dial = (current_dial + 100 - num_mod) % 100;
        } else {
            current_dial = (current_dial + num_mod) % 100;
        }

        password1a += count_zero(current_dial);
    }
    println!("{}", password1a);
    println!("{}", password1b);
}

/// Count how many times we pass through 0 when moving `steps` from `position` in given direction
fn count_zeros_on_the_way(dial: u32, dials: u32, is_left: bool) -> u32 {
    let first_zero = match (is_left, dial) {
        (_, 0) => 100,
        (true, d) => d,
        (false, d) => 100 - d,
    };

    if dials >= first_zero {
        (dials - first_zero) / 100 + 1
    } else {
        0
    }
}

fn count_zero(dial: u32) -> u32 {
    if dial == 0 { 1 } else { 0 }
}
