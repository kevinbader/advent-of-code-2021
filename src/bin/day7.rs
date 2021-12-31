use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day7.txt").expect("failed to read input file");
    let positions = parse(&input);
    let meeting_point = find_best_meeting_point(&positions);
    println!("Best meeting point: {}", meeting_point);
    let fuel_cost = fuel_cost(&positions, meeting_point);
    println!("Fuel costs: {}", fuel_cost);
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn find_best_meeting_point(positions: &[usize]) -> usize {
    // The fuel needed is the distance to the target position. To minimize the distance
    // for all crab submarines, we're looking for the median of all current positions.
    let sorted = {
        let mut cloned = positions.to_vec();
        cloned.sort_unstable();
        cloned
    };
    match sorted.len() {
        // if length is odd, we return the number in the middle:
        len if len % 2 != 0 => sorted[len / 2],
        // if length is even, we round the mean of the two middle numbers to the nearest integer:
        len => {
            let a = sorted[len / 2 - 1];
            let b = sorted[len / 2];
            ((a as f64 + b as f64) / 2.).round() as usize
        }
    }
}

fn fuel_cost(positions: &[usize], meeting_point: usize) -> usize {
    let mut cost = 0;
    for pos in positions {
        cost += abs_diff(*pos, meeting_point)
    }
    cost
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b.wrapping_sub(a)
    } else {
        a.wrapping_sub(b)
    }
}

#[cfg(test)]
mod example {
    use super::*;

    #[test]
    fn test_meeting_point() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let pos = find_best_meeting_point(&positions);
        assert_eq!(pos, 2);
    }

    #[test]
    fn test_fuel_cost() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let meeting_point = 2;
        let fuel_cost = fuel_cost(&positions, meeting_point);
        assert_eq!(fuel_cost, 37);
    }
}
