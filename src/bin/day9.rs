use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day9.txt").expect("failed to read input file");
    let low_points = low_points_of(&parse(&input));
    let risk_levels = risk_levels_of(&low_points);
    println!("risk levels: {:?}", risk_levels);
    println!("sum: {:?}", risk_levels.iter().sum::<u32>());
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
}
