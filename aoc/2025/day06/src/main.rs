use std::{fs::read_to_string, iter::Rev};

fn main() {
    println!("------------ 2025 Day 6 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let mut input_grid: Vec<Vec<String>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    // part 1
    input_grid.push(vec![String::new(); input_grid[0].len()]); // add empty row for operation results
    let mut input_grid_size = (input_grid.len(), input_grid[0].len());

    for col in 0..input_grid_size.1 {
        let operation = input_grid[input_grid_size.0 - 2][col].trim();
        let mut col_result = if operation == "*" { 1u64 } else { 0u64 };
        for row in (0..input_grid_size.0 - 2).rev() {
            let cell: u64 = input_grid[row][col].parse().unwrap();
            if operation == "*" {
                col_result *= cell;
            } else if operation == "+" {
                col_result += cell;
            }
        }
        input_grid[input_grid_size.0 - 1][col] = col_result.to_string();
    }

    let mut result = 0u64;
    for val in &input_grid[input_grid_size.0 - 1] {
        result += val.parse::<u64>().unwrap_or(0);
    }

    println!("{}", result);

    // part2
    // I have to re-parse the input since the whitespaces matter here
    let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let num_rows = char_grid.len();
    let num_cols = char_grid.iter().map(|r| r.len()).max().unwrap_or(0);

    let char_grid: Vec<Vec<char>> = char_grid
        .into_iter()
        .map(|mut row| {
            row.resize(num_cols, ' ');
            row
        })
        .collect();

    let mut columns: Vec<(usize, usize)> = vec![];
    let mut column_start: Option<usize> = None;

    for col in 0..num_cols {
        let is_separator = (0..num_rows).all(|row| char_grid[row][col] == ' ');
        if is_separator {
            if let Some(start) = column_start {
                columns.push((start, col - 1));
                column_start = None;
            }
        } else if column_start.is_none() {
            column_start = Some(col);
        }
    }
    if let Some(start) = column_start {
        columns.push((start, num_cols - 1));
    }

    let mut result2 = 0u64;

    for (start_col, end_col) in columns {
        let operation = (start_col..=end_col)
            .find_map(|c| {
                let ch = char_grid[num_rows - 1][c];
                if ch == '*' || ch == '+' {
                    Some(ch)
                } else {
                    None
                }
            })
            .unwrap();

        let mut column_result: u64 = if operation == '*' { 1 } else { 0 };

        for col in (start_col..=end_col).rev() {
            let number_str: String = (0..num_rows - 1)
                .filter_map(|row| {
                    let ch = char_grid[row][col];
                    if ch.is_ascii_digit() { Some(ch) } else { None }
                })
                .collect();

            if !number_str.is_empty() {
                let num: u64 = number_str.parse().unwrap();
                if operation == '*' {
                    column_result *= num;
                } else if operation == '+' {
                    column_result += num;
                }
            }
        }

        result2 += column_result;
    }

    println!("{}", result2);
}
