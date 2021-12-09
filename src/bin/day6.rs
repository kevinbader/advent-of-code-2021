use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day6.txt").expect("failed to read input file");
    let n_days = 80;
    let lanternfish = parse(&input);
    let lanternfish = advance(lanternfish, n_days);
    println!(
        "After {} days, there are {} lanternfish.",
        n_days,
        lanternfish.len()
    );
}

struct Lanternfish {
    timer: usize,
}

impl Lanternfish {
    fn advance_a_day(&mut self) -> Option<Lanternfish> {
        match self.timer {
            0 => {
                self.timer = 6;
                Some(Lanternfish { timer: 8 })
            }
            _ => {
                self.timer -= 1;
                None
            }
        }
    }
}

fn parse(input: &str) -> Vec<Lanternfish> {
    input
        .trim()
        .split(',')
        .map(|n| Lanternfish {
            timer: n.parse::<usize>().unwrap(),
        })
        .collect()
}

fn advance(mut lanternfish: Vec<Lanternfish>, n_days: usize) -> Vec<Lanternfish> {
    for _ in 0..n_days {
        let mut new_fish = vec![];
        for fish in &mut lanternfish {
            if let Some(baby_fish) = fish.advance_a_day() {
                new_fish.push(baby_fish);
            }
        }
        lanternfish.append(&mut new_fish);
    }
    lanternfish
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_lanternfish() {
        let lanternfish = parse(INPUT);
        let lanternfish = advance(lanternfish, 18);
        assert_eq!(lanternfish.len(), 26);
        let lanternfish = advance(lanternfish, 80 - 18);
        assert_eq!(lanternfish.len(), 5934);
    }
}
