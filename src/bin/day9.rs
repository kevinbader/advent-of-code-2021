use std::{collections::VecDeque, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input/day9.txt").expect("failed to read input file");
    let matrix = parse(&input);
    let low_points = low_points_of(&matrix);
    let risk_levels = risk_levels_of(&low_points);
    println!("[part1] sum: {:?}", risk_levels.iter().sum::<u32>());
    let basins = basins_in(&matrix);
    let score = three_largest_basins_size_product(&basins);
    println!("[part2] score: {:?}", score);
}

type RiskLevel = u32;

#[derive(Debug, Clone, Copy)]
struct Point {
    location: (usize, usize),
    value: u32,
}
impl Point {
    fn new(location: (usize, usize), value: u32) -> Self {
        Self { location, value }
    }
}
type Matrix = Vec<Vec<Point>>;

fn parse(input: &str) -> Matrix {
    let mut rows = vec![];
    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let row = line
            .chars()
            .map(|c| c.to_digit(10).expect("NaN"))
            .enumerate()
            .map(|(pos, val)| Point::new((pos, line_no), val))
            .collect();
        rows.push(row);
    }
    rows
}

#[allow(clippy::ptr_arg)]
fn low_points_of(matrix: &Matrix) -> Vec<Point> {
    let mut low_points = vec![];
    for (r, row) in matrix.iter().enumerate() {
        let has_neighbor_above = r > 0;
        let has_neighbor_below = r < (matrix.len() - 1);
        for (c, point) in row.iter().enumerate() {
            let has_left_neighbor = c > 0;
            let has_right_neighbor = c < (row.len() - 1);
            let mut field = vec![];
            if has_neighbor_above {
                field.push(matrix[r - 1][c]);
            }
            if has_neighbor_below {
                field.push(matrix[r + 1][c]);
            }
            if has_left_neighbor {
                field.push(matrix[r][c - 1]);
            }
            if has_right_neighbor {
                field.push(matrix[r][c + 1]);
            }
            // The point we're looking at is a low point iff all its neighbors are higher.
            if field.iter().all(|p| p.value > point.value) {
                low_points.push(*point);
            }
        }
    }
    low_points
}

fn risk_levels_of(values: &[Point]) -> Vec<RiskLevel> {
    values.iter().map(|x| x.value + 1).collect()
}

#[derive(Debug, Default)]
struct Basin<'a> {
    points: Vec<&'a Point>,
    size: usize,
}
impl<'a> Basin<'a> {
    fn add(&mut self, point: &'a Point) {
        self.points.push(point);
        self.size += 1;
    }
}

#[allow(clippy::ptr_arg)]
fn basins_in<'a>(matrix: &'a Matrix) -> Vec<Basin<'a>> {
    // We iterate through all of the points, but we remember and skip those already
    // visited. For each point we visit, we try to extend those the boundaries of the
    // basin (the '9's).
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut visited = vec![vec![false; n_cols]; n_rows];
    const BASIN_BOUNDARY: u32 = 9;
    let mut basins = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if visited[y][x] || point.value == BASIN_BOUNDARY {
                continue;
            }
            // This is a new basin
            let mut basin = Basin::default();
            let mut queue = VecDeque::from([point]);
            while let Some(p) = queue.pop_front() {
                // Have we been here already? Or is this a boundary?
                let (x, y) = p.location;
                if visited[y][x] || p.value == BASIN_BOUNDARY {
                    continue;
                }
                visited[y][x] = true;
                // We haven't seen this and it's not a boundary => it belongs to the basin!
                basin.add(p);
                // Add the neighbors to look at them later:
                if y > 0 {
                    queue.push_back(&matrix[y - 1][x]);
                }
                if y < (n_rows - 1) {
                    queue.push_back(&matrix[y + 1][x]);
                }
                if x > 0 {
                    queue.push_back(&matrix[y][x - 1]);
                }
                if x < (n_cols - 1) {
                    queue.push_back(&matrix[y][x + 1]);
                }
            }
            basins.push(basin);
        }
    }
    basins
}

fn three_largest_basins_size_product(basins: &[Basin]) -> usize {
    basins
        .iter()
        .map(|x| x.size)
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn test_low_points() {
        let low_points = low_points_of(&parse(INPUT));

        let risk_levels = risk_levels_of(&low_points);
        assert_eq!(risk_levels, vec![2, 1, 6, 6]);
        assert_eq!(risk_levels.iter().sum::<u32>(), 15);

        let low_points: Vec<u32> = low_points.iter().map(|x| x.value).collect();
        assert_eq!(low_points, vec![1, 0, 5, 5]);
    }

    #[test]
    fn test_basins() {
        let matrix = parse(INPUT);
        let basins = basins_in(&matrix);
        let score = three_largest_basins_size_product(&basins);
        assert_eq!(score, 1134);
    }
}
