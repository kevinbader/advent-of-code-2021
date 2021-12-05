use anyhow::{bail, Error};
use std::{fs};

fn main() {
    let input = fs::read_to_string("./input/day2.txt").expect("failed to read input file");
    let input = parse(&input).unwrap();
    let Distance {
        horizontal, depth, ..
    } = calculate_distance(&input);
    println!("{} * {} = {}", horizontal, depth, horizontal * depth);
}

#[derive(Default)]
struct Distance {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn from(s: &str) -> anyhow::Result<Self> {
        use Command::*;
        let parts = s.trim().split_whitespace().collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        let command = parts[0];
        let arg = parts[1].parse::<u32>()?;
        let command = match command {
            "forward" => Forward(arg),
            "down" => Down(arg),
            "up" => Up(arg),
            _ => bail!("unknown command: {}", command),
        };
        Ok(command)
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Command>> {
    input
        .trim()
        .lines()
        .map(|line| Command::from(line))
        .collect::<Result<Vec<_>, Error>>()
}

fn calculate_distance(input: &[Command]) -> Distance {
    use Command::*;
    input.iter().fold(Distance::default(), |acc, command| {
        let Distance {
            horizontal,
            depth,
            aim,
        } = acc;
        match command {
            Forward(units) => Distance {
                horizontal: horizontal + units,
                depth: depth + aim * units,
                ..acc
            },
            Down(units) => Distance {
                aim: aim + units,
                ..acc
            },
            Up(units) => Distance {
                aim: aim - units,
                ..acc
            },
        }
    })
}

#[test]
fn test_calculating_distance() {
    let input = "\
      forward 5
      down 5
      forward 8
      up 3
      down 8
      forward 2
    ";
    let input = parse(input).unwrap();
    let Distance {
        horizontal, depth, ..
    } = calculate_distance(&input);
    assert_eq!(horizontal, 15);
    assert_eq!(depth, 60);
}
