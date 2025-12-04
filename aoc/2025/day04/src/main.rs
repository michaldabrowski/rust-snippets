use std::fs::read_to_string;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, 0),
    (1, -1),
    (1, 1),
];

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let grid_size = (grid.len(), grid[0].len());
    println!("Grid size: {} rows x {} cols", grid_size.0, grid_size.1);

    // part 1
    let mut result = 0;
    for r in 0..grid_size.0 {
        for c in 0..grid_size.1 {
            let forklift_accessible = visit_cell(&grid, r, c, grid_size);
            if forklift_accessible {
                result += 1;
            }
        }
    }
    println!("Result: {}", result);

    // part 2
    let mut result_2 = 0;
    loop {
        for r in 0..grid_size.0 {
            for c in 0..grid_size.1 {
                let forklift_accessible = visit_cell_2(&mut grid, r, c, grid_size);
                if forklift_accessible {
                    result_2 += 1;
                }
            }
        }

        if !mark_visited_paper_rolls(&mut grid, grid_size) {
            break;
        }
    }
    println!("Result2: {}", result_2);
}

fn visit_cell(grid: &[Vec<char>], r: usize, c: usize, grid_size: (usize, usize)) -> bool {
    if grid[r][c] != '@' {
        return false;
    }

    let mut rolls_of_paper = 0;
    for (dr, dc) in DIRECTIONS.iter() {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nc >= 0 && nr < grid_size.0 as isize && nc < grid_size.1 as isize {
            let neighbor = grid[nr as usize][nc as usize];
            if neighbor == '@' {
                rolls_of_paper += 1;
            }
        }
    }

    rolls_of_paper < 4
}

// in place solution
fn visit_cell_2(grid: &mut [Vec<char>], r: usize, c: usize, grid_size: (usize, usize)) -> bool {
    if grid[r][c] != '@' {
        return false;
    }

    let mut rolls_of_paper = 0;
    for (dr, dc) in DIRECTIONS.iter() {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nc >= 0 && nr < grid_size.0 as isize && nc < grid_size.1 as isize {
            let neighbor = grid[nr as usize][nc as usize];
            if neighbor == '@' || neighbor == 'x' {
                rolls_of_paper += 1;
            }
        }
    }

    if rolls_of_paper < 4 {
        grid[r][c] = 'x'; // mark for removal
        true
    } else {
        false
    }
}

fn mark_visited_paper_rolls(grid: &mut [Vec<char>], grid_size: (usize, usize)) -> bool {
    let mut marked = false;
    for r in 0..grid_size.0 {
        for c in 0..grid_size.1 {
            if grid[r][c] == 'x' {
                grid[r][c] = '.'; // mark as visited
                marked = true;
            }
        }
    }

    marked
}
