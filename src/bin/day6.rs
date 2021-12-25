use std::fs;

use aoc::day6::{advance3, parse};

fn main() {
    let input = fs::read_to_string("./input/day6.txt").expect("failed to read input file");
    let n_days = 256;
    let lanternfish = parse(&input);
    let n_lanternfish = advance3(&lanternfish, n_days);
    println!(
        "After {} days, there are {} lanternfish.",
        n_days, n_lanternfish
    );
}
