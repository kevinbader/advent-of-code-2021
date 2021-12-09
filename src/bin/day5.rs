use std::cmp;
use std::fmt::Display;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day5.txt").expect("failed to read input file");
    let lines = parse(&input);
    let n_dangerous_areas = n_dangerous_areas(&lines);
    println!("There are {} dangerous areas.", n_dangerous_areas);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from(s: &str) -> Self {
        let nums: Vec<usize> = s
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        assert_eq!(nums.len(), 2);
        Self {
            x: nums[0],
            y: nums[1],
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Line(Coord, Coord);

impl Line {
    fn from(s: &str) -> Self {
        let coords: Vec<Coord> = s.trim().split("->").map(|s| Coord::from(s)).collect();
        assert_eq!(coords.len(), 2);
        Self(coords[0], coords[1])
    }

    fn max_x(&self) -> usize {
        if self.0.x > self.1.x {
            self.0.x
        } else {
            self.1.x
        }
    }

    fn max_y(&self) -> usize {
        if self.0.y > self.1.y {
            self.0.y
        } else {
            self.1.y
        }
    }

    fn alignment(&self) -> Alignment {
        use Alignment::*;
        match (self.0, self.1) {
            (Coord { y: y0, .. }, Coord { y: y1, .. }) if y0 == y1 => Horizontal,
            (Coord { x: x0, .. }, Coord { x: x1, .. }) if x0 == x1 => Vertical,
            (Coord { x: x0, y: y0 }, Coord { x: x1, y: y1 }) => {
                let x_diff = if x1 > x0 { x1 - x0 } else { x0 - x1 };
                let y_diff = if y1 > y0 { y1 - y0 } else { y0 - y1 };
                if x_diff != y_diff {
                    panic!("Not a 45% diagonal line: {}", self);
                }
                match (x1 > x0, y1 > y0) {
                    (true, true) => TopLeftBottomRight,
                    (true, false) => BottomLeftTopRight,
                    (false, true) => TopRightBottomLeft,
                    (false, false) => BottomRightTopLeft,
                }
            }
        }
    }

    fn points(&self) -> Vec<Coord> {
        use Alignment::*;
        match self.alignment() {
            Horizontal => {
                let y = self.0.y;
                let (x0, x1) = (self.0.x, self.1.x);
                let (left, right) = if x0 <= x1 {
                    (&self.0, &self.1)
                } else {
                    (&self.1, &self.0)
                };
                (left.x..=right.x).map(|x| Coord { x, y }).collect()
            }
            Vertical => {
                let x = self.0.x;
                let (y0, y1) = (self.0.y, self.1.y);
                let (top, bottom) = if y0 <= y1 {
                    (&self.0, &self.1)
                } else {
                    (&self.1, &self.0)
                };
                (top.y..=bottom.y).map(|y| Coord { x, y }).collect()
            }
            TopLeftBottomRight => {
                let diff = self.1.x - self.0.x;
                (0..=diff)
                    .map(|offset| Coord {
                        x: self.0.x + offset,
                        y: self.0.y + offset,
                    })
                    .collect()
            }
            BottomLeftTopRight => {
                let diff = self.1.x - self.0.x;
                (0..=diff)
                    .map(|offset| Coord {
                        x: self.0.x + offset,
                        y: self.0.y - offset,
                    })
                    .collect()
            }
            TopRightBottomLeft => {
                let diff = self.0.x - self.1.x;
                (0..=diff)
                    .map(|offset| Coord {
                        x: self.0.x - offset,
                        y: self.0.y + offset,
                    })
                    .collect()
            }
            BottomRightTopLeft => {
                let diff = self.0.x - self.1.x;
                (0..=diff)
                    .map(|offset| Coord {
                        x: self.0.x - offset,
                        y: self.0.y - offset,
                    })
                    .collect()
            }
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}

enum Alignment {
    Horizontal,
    Vertical,
    TopLeftBottomRight,
    BottomLeftTopRight,
    TopRightBottomLeft,
    BottomRightTopLeft,
}

fn parse(input: &str) -> Vec<Line> {
    input.trim().lines().map(|line| Line::from(line)).collect()
}

fn n_dangerous_areas(lines: &[Line]) -> usize {
    // Dangerous areas are points where two or more lines overlap.

    // How large is the playing field?
    let max_x: usize = lines
        .iter()
        .fold(0, |acc, &line| cmp::max(acc, line.max_x()));
    let max_y: usize = lines
        .iter()
        .fold(0, |acc, &line| cmp::max(acc, line.max_y()));

    let mut diagram: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for line in lines {
        let points = line.points();
        for point in points {
            diagram[point.x][point.y] += 1;
        }
    }

    diagram
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&count| *count >= 2)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "\
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        ";

    #[test]
    fn test_dangerous_areas() {
        let lines = parse(INPUT);
        assert_eq!(n_dangerous_areas(&lines), 12);
    }

    #[test]
    fn test_horizontal_line() {
        let line = Line::from("0,9 -> 5,9");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 0, y: 9 },
                Coord { x: 1, y: 9 },
                Coord { x: 2, y: 9 },
                Coord { x: 3, y: 9 },
                Coord { x: 4, y: 9 },
                Coord { x: 5, y: 9 }
            ]
        );
    }

    #[test]
    fn test_vertical_line() {
        let line = Line::from("5,0 -> 5,5");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 5, y: 0 },
                Coord { x: 5, y: 1 },
                Coord { x: 5, y: 2 },
                Coord { x: 5, y: 3 },
                Coord { x: 5, y: 4 },
                Coord { x: 5, y: 5 }
            ]
        );
    }

    #[test]
    fn test_diagonal_line_tlbr() {
        let line = Line::from("0,0 -> 5,5");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 2 },
                Coord { x: 3, y: 3 },
                Coord { x: 4, y: 4 },
                Coord { x: 5, y: 5 }
            ]
        );
    }

    #[test]
    fn test_diagonal_line_bltr() {
        let line = Line::from("0,5 -> 5,0");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 0, y: 5 },
                Coord { x: 1, y: 4 },
                Coord { x: 2, y: 3 },
                Coord { x: 3, y: 2 },
                Coord { x: 4, y: 1 },
                Coord { x: 5, y: 0 }
            ]
        );
    }

    #[test]
    fn test_diagonal_line_trbl() {
        let line = Line::from("5,0 -> 0,5");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 5, y: 0 },
                Coord { x: 4, y: 1 },
                Coord { x: 3, y: 2 },
                Coord { x: 2, y: 3 },
                Coord { x: 1, y: 4 },
                Coord { x: 0, y: 5 }
            ]
        );
    }

    #[test]
    fn test_diagonal_line_brtl() {
        let line = Line::from("5,5 -> 0,0");
        let points = line.points();
        assert_eq!(
            points,
            vec![
                Coord { x: 5, y: 5 },
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 3 },
                Coord { x: 2, y: 2 },
                Coord { x: 1, y: 1 },
                Coord { x: 0, y: 0 },
            ]
        );
    }
}
