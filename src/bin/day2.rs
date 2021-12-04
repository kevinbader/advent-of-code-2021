use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day2.txt").expect("failed to read input file");
    let input = parse(&input);
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

impl From<&str> for Command {
    fn from(line: &str) -> Self {
        use Command::*;
        let parts = line.trim().split_whitespace().collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        let command = parts[0];
        let arg = parts[1].parse::<u32>().expect("not u32");
        match command {
            "forward" => Forward(arg),
            "down" => Down(arg),
            "up" => Up(arg),
            _ => panic!("unknown command"),
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    input.trim().lines().map(|line| line.into()).collect()
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
    let input = parse(input);
    let Distance {
        horizontal, depth, ..
    } = calculate_distance(&input);
    assert_eq!(horizontal, 15);
    assert_eq!(depth, 60);
}
