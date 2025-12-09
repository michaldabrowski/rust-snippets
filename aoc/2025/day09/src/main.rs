use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 9 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let points: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().parse().unwrap();
            let y: i64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // part 1
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);

    // part 2 TODO
}
