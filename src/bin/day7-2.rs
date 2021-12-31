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
    let highest = *positions.iter().max().expect("no positions");
    let mut best_point = highest;
    let mut best_cost = usize::MAX;
    for meeting_point in 0..=highest {
        let cost = fuel_cost(positions, meeting_point);
        if cost < best_cost {
            best_point = meeting_point;
            best_cost = cost;
        }
    }
    best_point
}

fn fuel_cost(positions: &[usize], meeting_point: usize) -> usize {
    let mut cost = 0;
    for pos in positions {
        let distance = abs_diff(*pos, meeting_point);
        cost += (1..=distance).sum::<usize>();
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
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_fuel_cost_best_point() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let meeting_point = 5;
        let fuel_cost = fuel_cost(&positions, meeting_point);
        assert_eq!(fuel_cost, 168);
    }

    #[test]
    fn test_fuel_cost_prev_best_point() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let meeting_point = 2;
        let fuel_cost = fuel_cost(&positions, meeting_point);
        assert_eq!(fuel_cost, 206);
    }

    #[test]
    fn test_fuel_cost_individual_moves() {
        assert_eq!(fuel_cost(&[16], 5), 66);
        assert_eq!(fuel_cost(&[1], 5), 10);
        assert_eq!(fuel_cost(&[2], 5), 6);
        assert_eq!(fuel_cost(&[0], 5), 15);
        assert_eq!(fuel_cost(&[4], 5), 1);
        assert_eq!(fuel_cost(&[7], 5), 3);
        assert_eq!(fuel_cost(&[14], 5), 45);
    }
}
