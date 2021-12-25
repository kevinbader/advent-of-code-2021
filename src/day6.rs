#[derive(Debug, Clone)]
pub struct Lanternfish {
    pub timer: usize,
}

impl Lanternfish {
    pub fn new() -> Self {
        Self { timer: 8 }
    }

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

impl Default for Lanternfish {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse(input: &str) -> Vec<Lanternfish> {
    input
        .trim()
        .split(',')
        .map(|n| Lanternfish {
            timer: n.parse::<usize>().unwrap(),
        })
        .collect()
}

/// The OOP style implementation.
///
/// The fish know themselves how to get older and when to produce a baby fish. In each
/// iteration we add the new fish to the list of existing fish.
///
/// This works for up to 150 lanternfish, but takes way too long for 256 fish.
pub fn advance1(mut lanternfish: Vec<Lanternfish>, n_days: usize) -> Vec<Lanternfish> {
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

/// A "smarter" advance1.
///
/// Here we try to be clever by
///
/// - inlining the condition
/// - producing all offsprings at once by counting first and delaying struct creation
/// - not relying on vector's own resizing for capacity increase
///
/// Turns out, this is actually _slower_ than advance1! Try `cargo bench` to see the results.
pub fn advance2(mut lanternfish: Vec<Lanternfish>, n_days: usize) -> Vec<Lanternfish> {
    for _ in 0..n_days {
        let n_new_fish = lanternfish.iter().filter(|f| f.timer == 0).count();
        for fish in &mut lanternfish {
            fish.timer = if fish.timer > 0 { fish.timer - 1 } else { 6 };
        }
        lanternfish.resize(lanternfish.len() + n_new_fish, Lanternfish::new());
    }
    lanternfish
}

/// Just counts what needs counting.
///
/// This version counts exactly what we need to count, does not create any structs and
/// encodes the logic behind creating offsprings.
///
/// This is by a magnitude faster than the other two attempts and easily solves the 256-case.
pub fn advance3(lanternfish: &[Lanternfish], n_days: usize) -> usize {
    let mut counts: Vec<usize> = vec![0; 9];
    for fish in lanternfish {
        counts[fish.timer] += 1;
    }
    for _ in 0..n_days {
        let zero_timer_fish = counts[0];
        for i in 0..8 {
            counts[i] = counts[i + 1];
        }
        // All fish with count==0 produce a baby fish with a timer of 8:
        counts[8] = zero_timer_fish;
        // After that, their timer is reset to 6:
        counts[6] += zero_timer_fish;
    }
    counts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_advance1() {
        let lanternfish = parse(INPUT);
        let lanternfish = advance1(lanternfish, 18);
        assert_eq!(lanternfish.len(), 26);
        let lanternfish = advance1(lanternfish, 80 - 18);
        assert_eq!(lanternfish.len(), 5934);
    }

    #[test]
    fn test_advance2() {
        let lanternfish = parse(INPUT);
        let lanternfish = advance2(lanternfish, 18);
        assert_eq!(lanternfish.len(), 26);
        let lanternfish = advance2(lanternfish, 80 - 18);
        assert_eq!(lanternfish.len(), 5934);
    }

    #[test]
    fn test_advance3() {
        let lanternfish = parse(INPUT);
        let n_lanternfish = advance3(&lanternfish, 18);
        assert_eq!(n_lanternfish, 26);
        let n_lanternfish = advance3(&lanternfish, 80);
        assert_eq!(n_lanternfish, 5934);
    }
}
