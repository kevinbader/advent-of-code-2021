use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day10.txt").expect("failed to read input file");
    let score = score_for(&syntax_errors_in(&input));
    println!("score: {}", score);
}

enum ParseLineResult {
    Ok,
    Incomplete,
    SyntaxError(SyntaxError),
}
impl ParseLineResult {
    fn syntax_error(pos: usize, expected: char, actual: Option<char>) -> Self {
        Self::SyntaxError(SyntaxError {
            pos,
            expected,
            actual,
        })
    }
}

struct SyntaxError {
    pos: usize,
    expected: char,
    actual: Option<char>,
}

fn parse_line(line: &str) -> ParseLineResult {
    let mut stack = vec![];
    for (pos, symbol) in line.chars().enumerate() {
        match symbol {
            '(' | '[' | '{' | '<' => stack.push(symbol),
            ')' | ']' | '}' | '>' => {
                let actual = symbol;
                let expected = opening_symbol_for(symbol).unwrap();
                if let Some(corresponding_symbol) = stack.pop() {
                    if corresponding_symbol != expected {
                        return ParseLineResult::syntax_error(pos, expected, Some(actual));
                    }
                } else {
                    // Close symbol but no corresponding opening symbol - that's an error.
                    return ParseLineResult::syntax_error(pos, expected, Some(actual));
                }
            }
            // Ignoring any other character:
            _ => {}
        };
    }
    if stack.is_empty() {
        ParseLineResult::Ok
    } else {
        ParseLineResult::Incomplete
    }
}

fn first_syntax_error_in(line: &str) -> Option<SyntaxError> {
    match parse_line(line) {
        ParseLineResult::SyntaxError(syntax_error) => Some(syntax_error),
        _ => None,
    }
}

fn opening_symbol_for(symbol: char) -> Option<char> {
    match symbol {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn syntax_errors_in(text: &str) -> Vec<SyntaxError> {
    text.lines().filter_map(first_syntax_error_in).collect()
}

fn score_for(syntax_errors: &[SyntaxError]) -> usize {
    syntax_errors
        .iter()
        .map(|e| e.actual)
        .map(|c| match c {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => panic!("no score for symbol {:?}", c),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn test_part1() {
        let score = score_for(&syntax_errors_in(INPUT));
        assert_eq!(score, 26397);
    }
}
