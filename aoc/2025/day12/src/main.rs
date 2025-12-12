use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 12 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let sections: Vec<&str> = input.split("\n\n").collect();
    let shapes_section = sections[..sections.len() - 1].join("\n\n");
    let regions_section = sections.last().unwrap();

    let shapes = parse_shapes(&shapes_section);
    let regions = parse_regions(regions_section);

    let shape_sizes: Vec<usize> = shapes.iter().map(|s| s.len()).collect();
    let all_orientations: Vec<Vec<Shape>> = shapes.iter().map(get_all_orientations).collect();

    let mut count = 0;
    for (width, height, counts) in &regions {
        if can_fit_all(*width, *height, counts, &all_orientations, &shape_sizes) {
            count += 1;
        }
    }

    println!("{}", count);
}

type Shape = Vec<(i32, i32)>;

fn parse_shapes(section: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut current_shape: Vec<(i32, i32)> = Vec::new();
    let mut row = 0;

    for line in section.lines() {
        if line.contains(':') {
            if !current_shape.is_empty() {
                shapes.push(current_shape);
                current_shape = Vec::new();
            }
            row = 0;
        } else if !line.is_empty() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    current_shape.push((row, col as i32));
                }
            }
            row += 1;
        }
    }
    if !current_shape.is_empty() {
        shapes.push(current_shape);
    }
    shapes
}

fn normalize(shape: &Shape) -> Shape {
    let min_r = shape.iter().map(|&(r, _)| r).min().unwrap_or(0);
    let min_c = shape.iter().map(|&(_, c)| c).min().unwrap_or(0);
    let mut normalized: Shape = shape.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    normalize(&shape.iter().map(|&(r, c)| (c, -r)).collect())
}

fn flip_horizontal(shape: &Shape) -> Shape {
    normalize(&shape.iter().map(|&(r, c)| (r, -c)).collect())
}

fn get_all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = normalize(shape);

    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    current = flip_horizontal(shape);
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn parse_regions(section: &str) -> Vec<(usize, usize, Vec<usize>)> {
    let mut regions = Vec::new();
    for line in section.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let dims: Vec<usize> = parts[0]
            .split('x')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let counts: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        regions.push((dims[0], dims[1], counts));
    }
    regions
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    first_empty_row: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![vec![false; width]; height],
            first_empty_row: 0,
        }
    }

    fn can_place(&self, shape: &Shape, start_r: i32, start_c: i32) -> bool {
        for &(dr, dc) in shape {
            let r = start_r + dr;
            let c = start_c + dc;
            if r < 0 || c < 0 || r >= self.height as i32 || c >= self.width as i32 {
                return false;
            }
            if self.cells[r as usize][c as usize] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, start_r: i32, start_c: i32) {
        for &(dr, dc) in shape {
            let r = (start_r + dr) as usize;
            let c = (start_c + dc) as usize;
            self.cells[r][c] = true;
        }
    }

    fn remove(&mut self, shape: &Shape, start_r: i32, start_c: i32) {
        for &(dr, dc) in shape {
            let r = (start_r + dr) as usize;
            let c = (start_c + dc) as usize;
            self.cells[r][c] = false;
        }
    }

    fn first_empty(&mut self) -> Option<(i32, i32)> {
        for r in self.first_empty_row..self.height {
            for c in 0..self.width {
                if !self.cells[r][c] {
                    self.first_empty_row = r;
                    return Some((r as i32, c as i32));
                }
            }
        }
        None
    }

    fn reset_first_empty(&mut self, row: usize) {
        if row < self.first_empty_row {
            self.first_empty_row = row;
        }
    }
}

fn can_fit_all(
    width: usize,
    height: usize,
    counts: &[usize],
    all_orientations: &[Vec<Shape>],
    shape_sizes: &[usize],
) -> bool {
    let total_cells = width * height;

    let cells_needed: usize = counts
        .iter()
        .zip(shape_sizes.iter())
        .map(|(&count, &size)| count * size)
        .sum();

    if cells_needed > total_cells {
        return false;
    }

    let slack = total_cells - cells_needed;
    let total_shapes: usize = counts.iter().sum();

    if slack >= total_shapes * 2 {
        return true;
    }

    let mut grid = Grid::new(width, height);

    let mut shapes_to_place: Vec<(usize, &Vec<Shape>)> = counts
        .iter()
        .zip(all_orientations.iter())
        .map(|(&count, orients)| (count, orients))
        .collect();

    shapes_to_place.sort_by(|a, b| {
        let size_a = a.1.first().map(|s| s.len()).unwrap_or(0);
        let size_b = b.1.first().map(|s| s.len()).unwrap_or(0);
        size_b.cmp(&size_a)
    });

    solve(&mut grid, &mut shapes_to_place, total_cells, cells_needed)
}

fn solve(
    grid: &mut Grid,
    shapes_to_place: &mut [(usize, &Vec<Shape>)],
    cells_remaining: usize,
    cells_needed: usize,
) -> bool {
    if cells_remaining < cells_needed {
        return false;
    }

    let shape_idx = shapes_to_place.iter().position(|(count, _)| *count > 0);

    let shape_idx = match shape_idx {
        Some(idx) => idx,
        None => return true,
    };

    let first_empty = match grid.first_empty() {
        Some(pos) => pos,
        None => return false,
    };

    let orientations = shapes_to_place[shape_idx].1;
    let shape_size = orientations[0].len();

    for orientation in orientations.iter() {
        for &(dr, dc) in orientation.iter() {
            let start_r = first_empty.0 - dr;
            let start_c = first_empty.1 - dc;

            if grid.can_place(orientation, start_r, start_c) {
                grid.place(orientation, start_r, start_c);
                shapes_to_place[shape_idx].0 -= 1;

                if solve(
                    grid,
                    shapes_to_place,
                    cells_remaining - shape_size,
                    cells_needed - shape_size,
                ) {
                    return true;
                }

                shapes_to_place[shape_idx].0 += 1;
                grid.remove(orientation, start_r, start_c);
                grid.reset_first_empty(start_r as usize);
            }
        }
    }

    false
}
