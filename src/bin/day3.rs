use std::{fs, num::ParseIntError, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input/day3.txt").expect("failed to read input file");
    let input = input.parse().unwrap();

    let (gamma_rate, epsilon_rate) = gamma_and_epsilon_rates(&input);
    println!("gamma rate: {}", gamma_rate.0);
    println!("epsilon rate: {}", epsilon_rate.0);
    let power_consumption = power_consumption(gamma_rate, epsilon_rate);
    println!("power consumption: {}", power_consumption.0);

    let oxygen_rating = oxygen_generator_rating(&input);
    println!("oxygen rating: {}", oxygen_rating.0);
    let co2_scrubber_rating = co2_scrubber_rating(&input);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating.0);
    let life_support_rating = life_support_rating(oxygen_rating, co2_scrubber_rating);
    println!("life support rating: {}", life_support_rating.0);
}

struct Input {
    n_diagnostic_bits: u32,
    values: Vec<u32>,
}

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.trim().lines().map(|line| line.trim()).collect();
        assert!(!lines.is_empty());
        let n_chars = lines[0].len();
        assert!(n_chars <= u32::BITS.try_into().unwrap());
        let n_diagnostic_bits = n_chars as u32;
        let values = lines
            .iter()
            .map(|line| u32::from_str_radix(line, 2))
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        Ok(Self {
            n_diagnostic_bits,
            values,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct GammaRate(u32);
#[derive(Debug, Clone, Copy)]
struct EpsilonRate(u32);
#[derive(Debug, Clone, Copy)]
struct PowerConsumption(u32);
#[derive(Debug, Clone, Copy)]
struct OxygenGeneratorRating(u32);
#[derive(Debug, Clone, Copy)]
struct Co2ScrubberRating(u32);
#[derive(Debug, Clone, Copy)]
struct LifeSupportRating(u32);

fn gamma_and_epsilon_rates(
    Input {
        n_diagnostic_bits,
        values,
    }: &Input,
) -> (GammaRate, EpsilonRate) {
    let mut gamma_rate = GammaRate(0);
    let n_values = values.len();
    // The mask is used to scan individual bits, one after the other. We start with the first, most-left bit.
    let mut mask = 1 << (*n_diagnostic_bits - 1);
    for _ in 0..*n_diagnostic_bits {
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
    let epsilon_rate = EpsilonRate(!gamma_rate.0 & ((1 << *n_diagnostic_bits) - 1));
    (gamma_rate, epsilon_rate)
}

fn power_consumption(gamma_rate: GammaRate, epsilon_rate: EpsilonRate) -> PowerConsumption {
    PowerConsumption(gamma_rate.0 * epsilon_rate.0)
}

fn oxygen_generator_rating(
    Input {
        n_diagnostic_bits,
        values,
    }: &Input,
) -> OxygenGeneratorRating {
    let mut values = values.clone();
    let mut mask = 1 << (*n_diagnostic_bits - 1);
    for _ in 0..*n_diagnostic_bits {
        // Only retain values that share the most common bit value at this position.
        // If 0 and 1 are equally common, keep values with a 1 in the position being considered.
        let (values_with_low_bit, values_with_high_bit) = split_by_bit(values, mask);
        values = match (values_with_low_bit.len(), values_with_high_bit.len()) {
            (0, 0) => panic!("no values left?!"),
            (0, _) => values_with_high_bit,
            (_, 0) => values_with_low_bit,
            (n_low, n_high) if n_low > n_high => values_with_low_bit,
            (n_low, n_high) if n_low <= n_high => values_with_high_bit,
            _ => unreachable!(),
        };
        assert!(!values.is_empty());
        if values.len() == 1 {
            // Found it!
            return OxygenGeneratorRating(values[0]);
        }
        mask >>= 1;
    }
    panic!("spec doesn't say what we should do with more than one value left");
}

fn split_by_bit(values: Vec<u32>, mask: u32) -> (Vec<u32>, Vec<u32>) {
    let mut acc = (vec![], vec![]);
    for value in values {
        match value & mask {
            0 => acc.0.push(value),
            _ => acc.1.push(value),
        };
    }
    acc
}

fn co2_scrubber_rating(
    Input {
        n_diagnostic_bits,
        values,
    }: &Input,
) -> Co2ScrubberRating {
    let mut values = values.clone();
    let mut mask = 1 << (*n_diagnostic_bits - 1);
    for _ in 0..*n_diagnostic_bits {
        // Only retain values that share the least common bit value at this position.
        // If 0 and 1 are equally common, keep values with a 0 in the position being considered.
        let (values_with_low_bit, values_with_high_bit) = split_by_bit(values, mask);
        values = match (values_with_low_bit.len(), values_with_high_bit.len()) {
            (0, 0) => panic!("no values left?!"),
            (0, _) => values_with_high_bit,
            (_, 0) => values_with_low_bit,
            (n_low, n_high) if n_low <= n_high => values_with_low_bit,
            (n_low, n_high) if n_low > n_high => values_with_high_bit,
            _ => unreachable!(),
        };
        if values.len() == 1 {
            // Found it!
            return Co2ScrubberRating(values[0]);
        }
        mask >>= 1;
    }
    panic!("spec doesn't say what we should do with more than one value left");
}

fn life_support_rating(
    oxygen_generator_rating: OxygenGeneratorRating,
    co2_scrubber_rating: Co2ScrubberRating,
) -> LifeSupportRating {
    LifeSupportRating(oxygen_generator_rating.0 * co2_scrubber_rating.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computing_power_consumption() {
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
        let input = input.parse().unwrap();
        let (gamma_rate, epsilon_rate) = gamma_and_epsilon_rates(&input);
        let power_consumption = power_consumption(gamma_rate, epsilon_rate);
        assert_eq!(gamma_rate.0, 22);
        assert_eq!(epsilon_rate.0, 9);
        assert_eq!(power_consumption.0, 198);
    }

    #[test]
    fn test_oxygen_rating_uses_most_common_bits() {
        // For 2nd column, 0 is the most common bit initially, but after processing the 1st
        // column, the first two lines are discarded; after that, the most common value in
        // the 2nd column is 1.
        let input = "\
            000
            001
            111
            110
            101
            ";
        let input = input.parse().unwrap();
        let (gamma_rate, _) = gamma_and_epsilon_rates(&input);
        // Using the gamma_rate, we'd pick the wrong numbers:
        assert_eq!(gamma_rate.0, 0b101);
        let oxygen_rating = oxygen_generator_rating(&input);
        assert_eq!(oxygen_rating.0, 0b111);
    }

    #[test]
    fn test_oxygen_rating_prefers_high_bits() {
        let input = "\
            000
            111
            ";
        let input = input.parse().unwrap();
        let oxygen_rating = oxygen_generator_rating(&input);
        assert_eq!(oxygen_rating.0, 0b111);
    }

    #[test]
    fn test_co2_scrubber_rating_uses_least_common_bits() {
        // For 2nd column, 0 is the most common bit initially, but after processing the 1st
        // column, the first two lines are discarded; after that, the most common value in
        // the 2nd column is 1.
        let input = "\
            000
            001
            111
            110
            101
            ";
        let input = input.parse().unwrap();
        let (_, epsilon_rate) = gamma_and_epsilon_rates(&input);
        // Using the epsilon_rate, we'd pick the wrong numbers:
        assert_eq!(epsilon_rate.0, 0b010);
        let co2_scrubber_rating = co2_scrubber_rating(&input);
        assert_eq!(co2_scrubber_rating.0, 0b000);
    }

    #[test]
    fn test_co2_scrubber_rating_prefers_low_bits() {
        let input = "\
            000
            111
            ";
        let input = input.parse().unwrap();
        let oxygen_rating = co2_scrubber_rating(&input);
        assert_eq!(oxygen_rating.0, 0b000);
    }

    #[test]
    fn test_life_support_rating() {
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
        let input = input.parse().unwrap();
        let oxygen_rating = oxygen_generator_rating(&input);
        assert_eq!(oxygen_rating.0, 23);
        let co2_scrubber_rating = co2_scrubber_rating(&input);
        assert_eq!(co2_scrubber_rating.0, 10);
        let life_support_rating = life_support_rating(oxygen_rating, co2_scrubber_rating);
        assert_eq!(life_support_rating.0, 230);
    }
}
