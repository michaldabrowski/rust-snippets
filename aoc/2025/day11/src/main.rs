use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 11 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let device = parts[0];
            let outputs: Vec<&str> = parts[1].split_whitespace().collect();
            (device, outputs)
        })
        .collect();

    let count = count_paths(&graph, "you", &mut HashSet::new());
    println!("{}", count);

    let mut memo: HashMap<(&str, u8), u64> = HashMap::new(); // no cycles so we can use memoization
    let count2 = count_paths_with_required(&graph, "svr", 0, &mut memo);
    println!("{}", count2);
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if current == "out" {
        return 1;
    }

    if visited.contains(current) || !graph.contains_key(current) {
        return 0;
    }

    visited.insert(current);
    let mut total = 0;
    for &output in &graph[current] {
        total += count_paths(graph, output, visited);
    }
    visited.remove(current);

    total
}

fn count_paths_with_required<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    seen_mask: u8,
    memo: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    let mask =
        seen_mask | if current == "dac" { 1 } else { 0 } | if current == "fft" { 2 } else { 0 };

    if current == "out" {
        return if mask == 3 { 1 } else { 0 };
    }

    if let Some(&cached) = memo.get(&(current, mask)) {
        return cached;
    }

    if !graph.contains_key(current) {
        return 0;
    }

    let mut total = 0;
    for &output in &graph[current] {
        total += count_paths_with_required(graph, output, mask, memo);
    }

    memo.insert((current, mask), total);
    total
}
