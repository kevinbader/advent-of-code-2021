use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day1.txt").expect("failed to read input file");
    let input = parse(&input);
    let increases = measure(&input);
    println!("{}", increases);
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|s| s.trim().parse().expect("expected numbers only"))
        .collect()
}

fn measure(nums: &[i32]) -> i32 {
    let mut last = -1;
    let mut increases = -1;
    for n in nums {
        if *n > last {
            increases += 1;
        }
        last = *n;
    }
    increases
}

#[test]
fn sample1() {
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
    let increases = measure(&input);
    assert_eq!(increases, 7);
}
