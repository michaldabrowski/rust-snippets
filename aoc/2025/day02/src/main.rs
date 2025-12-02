use std::fs::read_to_string;

fn main() {
    let mut invalid_ids_sum: u64 = 0;
    let mut invalid_ids_sum_2: u64 = 0;

    let input = read_to_string("input.txt").expect("Failed to read input file");

    for split in input.trim().split(",") {
        if split.is_empty() {
            continue;
        }

        let range: Vec<&str> = split.split("-").collect();
        let id_start: u64 = range[0].parse().unwrap();
        let id_end: u64 = range[1].parse().unwrap();

        for id in id_start..=id_end {
            if is_invalid_id(id) {
                invalid_ids_sum += id;
            }
            if is_invalid_id_2(id) {
                invalid_ids_sum_2 += id;
            }
        }
    }

    println!("{}", invalid_ids_sum);
    println!("{}", invalid_ids_sum_2);
}

fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

fn is_invalid_id_2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[..pattern_len];
        if s.chars()
            .collect::<Vec<_>>()
            .chunks(pattern_len)
            .all(|chunk| chunk.iter().collect::<String>() == pattern)
        {
            return true;
        }
    }
    false
}
