use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day3.txt").expect("failed to read input file");
    let input = parse(&input);
    let (gamma_rate, epsilon_rate) = analyze(input);
    let power_consumption = power_consumption(gamma_rate, epsilon_rate);
    println!("{}", power_consumption.0);
}

struct Input {
    n_diagnostic_bits: u32,
    values: Vec<u32>,
}

fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.trim().lines().map(|line| line.trim()).collect();
    assert!(!lines.is_empty());
    let n_diagnostic_bits = lines[0].len() as u32;
    let values = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).expect("not binary or too large"))
        .collect();
    Input {
        n_diagnostic_bits,
        values,
    }
}

#[derive(Debug, Clone, Copy)]
struct GammaRate(u32);
#[derive(Debug, Clone, Copy)]
struct EpsilonRate(u32);
#[derive(Debug, Clone, Copy)]
struct PowerConsumption(u32);

fn analyze(
    Input {
        n_diagnostic_bits,
        values,
    }: Input,
) -> (GammaRate, EpsilonRate) {
    let mut gamma_rate = GammaRate(0);
    assert!(n_diagnostic_bits <= u32::BITS);
    let n_values = values.len();
    // The mask is used to scan individual bits, one after the other. We start with the first, most-left bit.
    let mut mask = 1 << (n_diagnostic_bits - 1);
    for _ in 0..n_diagnostic_bits {
        // For each input number, select the nth bit and count the non-zero numbers:
        let n_one_bits = values.iter().map(|n| n & mask).filter(|n| *n > 0).count();
        let most_common_bit = if n_one_bits > (n_values / 2) { 1 } else { 0 };
        if most_common_bit == 1 {
            // Toggle that bit on the gamma rate:
            gamma_rate.0 ^= mask;
        }
        mask >>= 1;
    }
    assert_eq!(mask, 0);
    // Epsilon is gamma, flipped. Since u32 has more bits, we strip off any additional ones using bitwise AND.
    let epsilon_rate = EpsilonRate(!gamma_rate.0 & ((1 << n_diagnostic_bits) - 1));
    (gamma_rate, epsilon_rate)
}

fn power_consumption(gamma_rate: GammaRate, epsilon_rate: EpsilonRate) -> PowerConsumption {
    PowerConsumption(gamma_rate.0 * epsilon_rate.0)
}

#[test]
fn test_calculating_power_consumption() {
    let input = "\
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";
    let input = parse(input);
    let (gamma_rate, epsilon_rate) = analyze(input);
    let power_consumption = power_consumption(gamma_rate, epsilon_rate);
    assert_eq!(gamma_rate.0, 22);
    assert_eq!(epsilon_rate.0, 9);
    assert_eq!(power_consumption.0, 198);
}
