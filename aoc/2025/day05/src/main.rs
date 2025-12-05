use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 5 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    // process input
    let mut fresh_ids_ranges: Vec<(u64, u64)> = Vec::new();
    let mut available_ids: Vec<u64> = Vec::new();
    let mut processed_ranges = false;
    for line in input.lines() {
        if line.is_empty() {
            processed_ranges = true;
            continue;
        }

        if !processed_ranges {
            process_range(&line, &mut fresh_ids_ranges);
        } else {
            available_ids.push(line.parse().unwrap())
        }
    }

    // merge ranges
    let merged_fresh_ids_ranges = merge_ranges(&mut fresh_ids_ranges);

    // count available fresh ids
    let available_fresh_ids = available_ids
        .iter()
        .filter(|&&id| is_fresh(id, &merged_fresh_ids_ranges))
        .count();

    // and count total available fresh ids
    let total_available_fresh_ids: u64 = merged_fresh_ids_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    println!("{}", available_fresh_ids);
    println!("{}", total_available_fresh_ids);
}

fn process_range(line: &str, fresh_ids: &mut Vec<(u64, u64)>) {
    let (start, end) = line.split_once('-').unwrap();
    fresh_ids.push((start.parse().unwrap(), end.parse().unwrap()));
}

fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = vec![ranges[0]];
    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    // Binary search for the range that could contain this ID
    let idx = ranges.partition_point(|r| r.1 < id);
    idx < ranges.len() && ranges[idx].0 <= id
}
