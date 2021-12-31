use std::{collections::HashMap, fmt::Display, fs};

fn main() {
    let input = fs::read_to_string("./input/day8.txt").expect("failed to read input file");
    let entries: Vec<Entry> = parse(&input);
    let mut total = 0;
    for entry in &entries {
        let decoder = Decoder::from(entry);
        let ones = decoder.count_in_output(Digit::One);
        let fours = decoder.count_in_output(Digit::Four);
        let sevens = decoder.count_in_output(Digit::Seven);
        let eights = decoder.count_in_output(Digit::Eight);
        let sum = ones + fours + sevens + eights;
        total += sum;
        println!("{}", decoder);
        println!("sum: {}", sum);
    }
    println!();
    println!("total: {}", total);
}

fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(Entry::from_str)
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SignalPattern(String);

impl SignalPattern {
    fn from(word: &str) -> Self {
        let mut chars: Vec<_> = word.to_lowercase().chars().collect();
        chars.sort_unstable();
        Self(chars.iter().collect())
    }
}

struct Entry {
    input: Vec<SignalPattern>,
    output: Vec<SignalPattern>,
}

impl Entry {
    fn from_str(line: &str) -> Self {
        let (input, output) = match line.split('|').collect::<Vec<&str>>().as_slice() {
            [input, output] => (*input, *output),
            split => panic!("unexpected format: {:?}", split),
        };
        let input: Vec<SignalPattern> = input
            .split_whitespace()
            .map(|x| SignalPattern::from(x))
            .collect();
        assert_eq!(input.len(), 10);
        let output: Vec<SignalPattern> = output
            .split_whitespace()
            .map(|x| SignalPattern::from(x))
            .collect();
        assert_eq!(output.len(), 4);
        Entry { input, output }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn from_str(signal_pattern: &SignalPattern) -> Option<Self> {
        use Digit::*;
        // Some digits are displayed using a unique number of segments:
        match signal_pattern.0.len() {
            2 => Some(One),
            4 => Some(Four),
            3 => Some(Seven),
            7 => Some(Eight),
            _ => None,
        }
    }
}

struct Decoder {
    pattern_to_digit: HashMap<SignalPattern, Digit>,
    input_digit_counts: HashMap<Digit, usize>,
    output_digit_counts: HashMap<Digit, usize>,
}

impl Decoder {
    fn from(entry: &Entry) -> Self {
        let mut decoder = Self {
            pattern_to_digit: HashMap::new(),
            input_digit_counts: HashMap::new(),
            output_digit_counts: HashMap::new(),
        };
        for input_pattern in &entry.input {
            if let Some(digit) = decoder.add(input_pattern) {
                *decoder.input_digit_counts.entry(digit).or_insert(0) += 1;
            }
        }
        for output_pattern in &entry.output {
            if let Some(digit) = decoder.add(output_pattern) {
                *decoder.output_digit_counts.entry(digit).or_insert(0) += 1;
            }
        }
        decoder
    }

    fn add(&mut self, signal_pattern: &SignalPattern) -> Option<Digit> {
        if self.pattern_to_digit.contains_key(signal_pattern) {
            Some(self.pattern_to_digit[signal_pattern])
        } else if let Some(digit) = Digit::from_str(signal_pattern) {
            self.pattern_to_digit.insert(signal_pattern.clone(), digit);
            Some(digit)
        } else {
            None
        }
    }

    fn count_in_input(&self, digit: Digit) -> usize {
        *self.input_digit_counts.get(&digit).unwrap_or(&0)
    }

    fn count_in_output(&self, digit: Digit) -> usize {
        *self.output_digit_counts.get(&digit).unwrap_or(&0)
    }
}

impl Display for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        for (pattern, digit) in &self.pattern_to_digit {
            lines.push(format!(
                "{:>7} => {:?} (occurrences: input={} output={})",
                pattern.0,
                digit,
                self.count_in_input(*digit),
                self.count_in_output(*digit)
            ));
        }
        write!(f, "{}", lines.join("\n"))
    }
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
    fn example() {
        let entries: Vec<Entry> = parse(INPUT);
        let mut total = 0;
        for entry in &entries {
            let decoder = Decoder::from(entry);
            let ones = decoder.count_in_output(Digit::One);
            let fours = decoder.count_in_output(Digit::Four);
            let sevens = decoder.count_in_output(Digit::Seven);
            let eights = decoder.count_in_output(Digit::Eight);
            let sum = ones + fours + sevens + eights;
            total += sum;
            println!("{}", decoder);
            println!("sum: {}", sum);
        }
        assert_eq!(total, 26);
    }
}
