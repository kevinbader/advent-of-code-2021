use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input/day8.txt").expect("failed to read input file");
    let total: u32 = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(decode_output)
        .sum();
    println!("total: {}", total);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pattern(String);

impl Pattern {
    fn from(word: &str) -> Self {
        let mut chars: Vec<_> = word.to_lowercase().chars().collect();
        chars.sort_unstable();
        Self(chars.iter().collect())
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn contains(&self, other: &Self) -> bool {
        other.0.chars().all(|c| self.0.contains(c))
    }
}

fn decode_output(line: &str) -> u32 {
    let (input, output) = line.split('|').collect_tuple().unwrap();
    let input = input.split_whitespace().map(Pattern::from).collect_vec();
    let output = output.split_whitespace().map(Pattern::from).collect_vec();

    let one = input.iter().find(|x| x.len() == 2).unwrap();
    let four = input.iter().find(|x| x.len() == 4).unwrap();
    let seven = input.iter().find(|x| x.len() == 3).unwrap();
    let eight = input.iter().find(|x| x.len() == 7).unwrap();
    // 6 is the only pattern of length 6 that doesn't contain 1 or 4.
    let six = input
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| !x.contains(one) && !x.contains(four))
        .unwrap();
    // 9 is the only pattern of length 6 that contains 4.
    let nine = input
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| x.contains(four))
        .unwrap();
    // 0 is the only pattern of length 6 that contains 1 but not 4.
    let zero = input
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| x.contains(one) && !x.contains(four))
        .unwrap();
    // 3 is the only pattern of length 5 that contains 1.
    let three = input
        .iter()
        .filter(|x| x.len() == 5)
        .find(|x| x.contains(one))
        .unwrap();
    // 2 is the only pattern of length 5 that doesn't contain 1 and also doesn't fit into 6.
    let two = input
        .iter()
        .filter(|x| x.len() == 5)
        .find(|x| !x.contains(one) && !six.contains(x))
        .unwrap();
    // 5 is the only pattern of length 5 that doesn't contain 1 and fits into 6.
    let five = input
        .iter()
        .filter(|x| x.len() == 5)
        .find(|x| !x.contains(one) && six.contains(x))
        .unwrap();

    let patterns = HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]);

    let output_value: String = output.iter().map(|p| patterns[&p].to_string()).collect();

    output_value.parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn example_part2() {
        let output_total: u32 = INPUT
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(decode_output)
            .sum();
        assert_eq!(output_total, 61229);
    }

    #[test]
    fn test_single_entry() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let output = decode_output(line);
        assert_eq!(output, 5353);
    }
}
