use std::fs::read_to_string;

fn main() {
    let mut invalid_ids_sum: u64 = 0;

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
        }
    }

    println!("{}", invalid_ids_sum)
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
