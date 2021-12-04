use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day1.txt").expect("failed to read input file");
    let input = parse(&input);
    println!(
        "with individual numbers: {}",
        measure_looking_at_individual_numbers(&input)
    );
    println!(
        "with sliding windows of 3: {}",
        measure_looking_at_sliding_windows_of_three(&input)
    );
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|s| s.trim().parse().expect("expected numbers only"))
        .collect()
}

fn measure_looking_at_individual_numbers(nums: &[i32]) -> usize {
    nums.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

fn measure_looking_at_sliding_windows_of_three(nums: &[i32]) -> usize {
    nums
        // Look at 3 numbers at a time:
        .windows(3)
        // Sum them up:
        .map(|window| window.iter().sum::<i32>())
        // On those sums, look at 2 numbers at a time:
        .collect::<Vec<i32>>()
        .windows(2)
        // Retain where the second number is higher => that's an increase!
        .filter(|pair| pair[1] > pair[0])
        // Count the number of increases:
        .count()
}

#[test]
fn test_measuring_by_looking_at_individual_numbers() {
    let input = "\
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
    let input = parse(input);
    let increases = measure_looking_at_individual_numbers(&input);
    assert_eq!(increases, 7);
}

#[test]
fn test_measuring_by_looking_at_sliding_windows_of_three() {
    let input = "\
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
    let input = parse(input);
    let increases = measure_looking_at_sliding_windows_of_three(&input);
    assert_eq!(increases, 5);
}
