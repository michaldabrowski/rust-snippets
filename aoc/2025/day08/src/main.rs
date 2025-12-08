use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 8 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    // parse points from input
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        let coords: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let point = Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };
        points.push(point);
    }

    // precompute distances between all points
    let precomputed_distances: Vec<Vec<f64>> = (0..points.len())
        .map(|i| {
            (0..points.len())
                .map(|j| euclidean_distance(&points[i], &points[j]))
                .collect()
        })
        .collect();

    // build circuits until all points are connected in a single circuit
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut connected_edges: HashSet<(usize, usize)> = HashSet::new();

    let mut connection_count = 0;
    let mut part1_result = 0;
    let mut part2_result = 0;

    while circuits.len() != 1 || circuits.is_empty() || circuits[0].len() != points.len() {
        // get next min edge
        let (point_i_idx, point_j_idx) = min_value(&precomputed_distances, &connected_edges);

        let edge = if point_i_idx < point_j_idx {
            (point_i_idx, point_j_idx)
        } else {
            (point_j_idx, point_i_idx)
        };
        connected_edges.insert(edge);

        let mut circuit_i: Option<usize> = None;
        let mut circuit_j: Option<usize> = None;
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&point_i_idx) {
                circuit_i = Some(idx);
            }
            if circuit.contains(&point_j_idx) {
                circuit_j = Some(idx);
            }
        }

        // add edge to circuits
        match (circuit_i, circuit_j) {
            (None, None) => {
                // Neither point in a circuit - create new one
                circuits.push(vec![point_i_idx, point_j_idx]);
            }
            (Some(ci), None) => {
                // Only point_i in a circuit - add point_j to it
                circuits[ci].push(point_j_idx);
            }
            (None, Some(cj)) => {
                // Only point_j in a circuit - add point_i to it
                circuits[cj].push(point_i_idx);
            }
            (Some(ci), Some(cj)) if ci != cj => {
                // Different circuits - merge them
                // Remove the higher index first to avoid shifting issues
                let higher = ci.max(cj);
                let lower = ci.min(cj);
                let circuit_to_merge = circuits.remove(higher);
                circuits[lower].extend(circuit_to_merge);
            }
            (Some(_), Some(_)) => {
                // Same circuit - do nothing
            }
        }

        connection_count += 1;

        if connection_count == 1000 {
            let mut sorted_circuits = circuits.clone();
            sorted_circuits.sort_by(|a, b| b.len().cmp(&a.len()));
            part1_result =
                sorted_circuits[0].len() * sorted_circuits[1].len() * sorted_circuits[2].len();
        }

        if circuits.len() == 1 && circuits[0].len() == points.len() {
            part2_result = points[point_i_idx].x * points[point_j_idx].x;
        }
    }

    println!("{}", part1_result);
    println!("{}", part2_result);
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn min_value(distances: &Vec<Vec<f64>>, connected: &HashSet<(usize, usize)>) -> (usize, usize) {
    let mut min_dist = f64::MAX;
    let mut min_i = 0;
    let mut min_j = 0;

    for i in 0..distances.len() {
        for j in (i + 1)..distances.len() {
            let edge = (i, j);
            if distances[i][j] < min_dist && !connected.contains(&edge) {
                min_dist = distances[i][j];
                min_i = i;
                min_j = j;
            }
        }
    }

    (min_i, min_j)
}

fn euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x - p2.x).pow(2);
    let dy = (p1.y - p2.y).pow(2);
    let dz = (p1.z - p2.z).pow(2);
    ((dx + dy + dz) as f64).sqrt()
}
