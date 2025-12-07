use std::collections::HashMap;
use std::fs::read_to_string;

const START: char = 'S';
const SPLITTER: char = '^';

fn main() {
    println!("------------ 2025 Day 7 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start_col = grid
        .iter()
        .flat_map(|row| row.iter())
        .position(|&c| c == START)
        .map(|pos| pos % cols)
        .unwrap_or_else(|| panic!("No starting position {} found", START));

    let mut beam_positions: HashMap<usize, u64> = HashMap::from([(start_col, 1)]);
    let mut split_count = 0;
    for row_idx in 1..rows {
        let mut next_positions: HashMap<usize, u64> = HashMap::new();

        for (&col, &timeline_count) in &beam_positions {
            if grid[row_idx][col] == SPLITTER {
                split_count += 1;
                // Split beam - goes left and right
                if col > 0 {
                    *next_positions.entry(col - 1).or_default() += timeline_count;
                }
                if col + 1 < cols {
                    *next_positions.entry(col + 1).or_default() += timeline_count;
                }
            } else {
                // Continue down
                *next_positions.entry(col).or_default() += timeline_count;
            }
        }

        beam_positions = next_positions;

        if beam_positions.is_empty() {
            break;
        }
    }

    let total_timelines: u64 = beam_positions.values().sum();
    println!("{}", split_count);
    println!("{}", total_timelines);
}
