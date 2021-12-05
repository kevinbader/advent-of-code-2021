use std::{collections::HashMap, fs, num::ParseIntError};

use anyhow::Context;

fn main() {
    let input = fs::read_to_string("./input/day4.txt").expect("failed to read input file");

    let (boards, drawn_numbers) = parse(&input).unwrap();
    match play_to_win(boards, drawn_numbers) {
        BingoResult::Bingo { board_no, score } => {
            println!(
                "Board #{} is the first one to have BINGO with score {}",
                board_no, score
            )
        }
        BingoResult::NoBingo => println!("No board has BINGO"),
    };

    let (boards, drawn_numbers) = parse(&input).unwrap();
    match play_to_lose(boards, drawn_numbers) {
        BingoResult::Bingo { board_no, score } => {
            println!(
                "Board #{} is the last one to have BINGO with score {}",
                board_no, score
            )
        }
        BingoResult::NoBingo => println!("No board has BINGO"),
    };
}

struct DrawnNumbers(Vec<u32>);
impl DrawnNumbers {
    fn try_from(line: &str) -> anyhow::Result<Self> {
        let nums = line
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;
        Ok(Self(nums))
    }
}

struct Board {
    // 5x5 board.
    fields: [[u32; 5]; 5],
    // To quickly check whether a number is on the board; maps number to (row,col).
    field_set: HashMap<u32, (usize, usize)>,
    // To mark drawn numbers.
    marks: [[bool; 5]; 5],
    // Only set if BINGO.
    score: Option<u32>,
}
impl Board {
    pub fn try_from(lines: &[&str]) -> anyhow::Result<Self> {
        let mut board = Board {
            fields: [[0; 5]; 5],
            marks: [[false; 5]; 5],
            field_set: HashMap::new(),
            score: None,
        };
        for (line_no, line) in lines.iter().enumerate() {
            assert!(line_no < 5);
            for (i, num) in line.split_whitespace().enumerate() {
                assert!(i < 5);
                let num = num.parse()?;
                board.fields[line_no][i] = num;
                board.field_set.insert(num, (line_no, i));
            }
        }
        Ok(board)
    }

    pub fn mark(&mut self, num: u32) -> BoardResult {
        if let Some((row, col)) = self.field_set.get(&num) {
            let row = *row;
            let col = *col;

            self.marks[row][col] = true;

            if self.is_bingo() {
                let score = self.comp_score(num);
                self.score = Some(score);
                BoardResult::Bingo { score }
            } else {
                BoardResult::NoBingo
            }
        } else {
            BoardResult::NoBingo
        }
    }

    fn is_bingo(&self) -> bool {
        // horizontal:
        for row in 0..5 {
            if self.marks[row].iter().all(|x| *x) {
                return true;
            }
        }
        // vertical:
        for col in 0..5 {
            if (0..5).all(|row| self.marks[row][col]) {
                return true;
            }
        }
        false
    }

    fn comp_score(&self, last_num: u32) -> u32 {
        // Sum of all unmarked numbers, multiplied by the last number:
        let mut sum_unmarked = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marks[row][col] {
                    sum_unmarked += self.fields[row][col];
                }
            }
        }
        sum_unmarked * last_num
    }
}

enum BoardResult {
    Bingo { score: u32 },
    NoBingo,
}

fn parse(s: &str) -> anyhow::Result<(Vec<Board>, DrawnNumbers)> {
    let mut lines = s.lines();
    // first line is drawn numbers:
    let drawn_numbers =
        DrawnNumbers::try_from(lines.next().map(|l| l.trim()).expect("no lines to parse"))
            .context("Failed to parse the first line")?;
    // every 5 lines is a board; skip empty lines
    let boards = lines
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks_exact(5)
        .map(|lines| Board::try_from(lines))
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    Ok((boards, drawn_numbers))
}

fn play_to_win(mut boards: Vec<Board>, drawn_numbers: DrawnNumbers) -> BingoResult {
    for num in drawn_numbers.0 {
        for (i, board) in boards.iter_mut().enumerate() {
            if let BoardResult::Bingo { score } = board.mark(num) {
                return BingoResult::Bingo {
                    board_no: i + 1,
                    score,
                };
            }
        }
    }
    BingoResult::NoBingo
}

fn play_to_lose(mut boards: Vec<Board>, drawn_numbers: DrawnNumbers) -> BingoResult {
    for num in drawn_numbers.0 {
        let remaining_boards: Vec<(usize, &mut Board)> = boards
            .iter_mut()
            .enumerate()
            .filter(|(_, board)| board.score.is_none())
            .collect();
        let is_last_board = remaining_boards.len() == 1;
        for (i, board) in remaining_boards {
            if let BoardResult::Bingo { score } = board.mark(num) {
                if is_last_board {
                    return BingoResult::Bingo {
                        board_no: i + 1,
                        score,
                    };
                }
            }
        }
    }
    BingoResult::NoBingo
}

enum BingoResult {
    Bingo { board_no: usize, score: u32 },
    NoBingo,
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "\
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        
        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
        ";

    #[test]
    fn test_play_to_win() {
        let (boards, drawn_numbers) = parse(INPUT).unwrap();
        match play_to_win(boards, drawn_numbers) {
            BingoResult::Bingo { board_no, score } => {
                assert_eq!(board_no, 3);
                assert_eq!(score, 4512);
            }
            _ => panic!("bingo expected"),
        };
    }

    #[test]
    fn test_play_to_lose() {
        let (boards, drawn_numbers) = parse(INPUT).unwrap();
        match play_to_lose(boards, drawn_numbers) {
            BingoResult::Bingo { board_no, score } => {
                assert_eq!(board_no, 2);
                assert_eq!(score, 1924);
            }
            _ => panic!("bingo expected"),
        };
    }
}
