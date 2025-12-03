use std::fs::read_to_string;

fn main() {
    let mut total_output: u64 = 0;
    let mut total_output_2: u64 = 0;
    let input = read_to_string("input.txt").expect("Failed to read input file");

    for line in input.lines() {
        total_output += find_max_joltage(line.as_bytes()) as u64;
        total_output_2 += find_max_joltage_k(line.as_bytes(), 12) as u64;
    }

    println!("{}", total_output);
    println!("{}", total_output_2);
}

fn find_max_joltage(bytes: &[u8]) -> u8 {
    let mut max_joltage = 0u8;
    let mut max_right = bytes[bytes.len() - 1] - b'0';

    for i in (0..bytes.len() - 1).rev() {
        let digit = bytes[i] - b'0';
        let joltage = digit * 10 + max_right;
        if joltage == 99 {
            return 99;
        }
        max_joltage = max_joltage.max(joltage);
        max_right = max_right.max(digit);
    }

    max_joltage
}

fn find_max_joltage_k(bytes: &[u8], k: usize) -> u64 {
    let n = bytes.len();
    let mut max_joltage: u64 = 0;
    let mut start = 0;

    for remaining in (1..=k).rev() {
        let end = n - remaining;

        // Find position of max digit in [start, end], preferring leftmost on ties
        let mut max_pos = start;
        for i in (start + 1)..=end {
            if bytes[i] > bytes[max_pos] {
                max_pos = i;
            }
        }

        max_joltage = max_joltage * 10 + (bytes[max_pos] - b'0') as u64;
        start = max_pos + 1;
    }

    max_joltage
}
