use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day10.txt").expect("failed to read input file");
    let score = score_for(&syntax_errors_in(&input));
    println!("part 1 syntax error score: {}", score);
    let incomplete_lines = incomplete_lines_in(&input);
    let completion_score = completion_score_of(&incomplete_lines);
    println!("part 2 completion score: {}", completion_score);
}

enum ParseLineResult<'a> {
    Ok,
    Incomplete(IncompleteLine<'a>),
    SyntaxError(SyntaxError),
}
impl<'a> ParseLineResult<'a> {
    fn syntax_error(pos: usize, expected: char, actual: Option<char>) -> Self {
        Self::SyntaxError(SyntaxError {
            pos,
            expected,
            actual,
        })
    }
    fn incomplete_line(line: &'a str, open_symbols: Vec<char>) -> Self {
        Self::Incomplete(IncompleteLine { line, open_symbols })
    }
}

struct SyntaxError {
    pos: usize,
    expected: char,
    actual: Option<char>,
}

struct IncompleteLine<'a> {
    line: &'a str,
    open_symbols: Vec<char>,
}
impl<'a> IncompleteLine<'a> {
    fn completed(&self) -> String {
        format!("{}{}", self.line, self.completion())
    }
    fn score(&self) -> usize {
        let mut score = 0;
        for c in self.completion().chars() {
            // For each character, multiply the total score by 5..
            score *= 5;
            // ..and then increase it by the character's value.
            score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("no score defined for char used for completion: {}", c),
            };
        }
        score
    }
    fn completion(&self) -> String {
        self.open_symbols
            .iter()
            .rev()
            .filter_map(|c| closing_symbol_for(*c))
            .collect::<String>()
    }
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
        ParseLineResult::incomplete_line(line, stack)
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

fn closing_symbol_for(symbol: char) -> Option<char> {
    match symbol {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

// Part 1
fn syntax_errors_in(text: &str) -> Vec<SyntaxError> {
    text.lines()
        .map(|l| l.trim())
        .filter_map(|line| match parse_line(line) {
            ParseLineResult::SyntaxError(syntax_error) => Some(syntax_error),
            _ => None,
        })
        .collect()
}

// Part 2
fn incomplete_lines_in(text: &str) -> Vec<IncompleteLine> {
    text.lines()
        .map(|l| l.trim())
        .filter_map(|line| match parse_line(line) {
            ParseLineResult::Incomplete(incomplete_line) => Some(incomplete_line),
            _ => None,
        })
        .collect()
}

// Part 1
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

// Part 2
fn completion_score_of(incomplete_lines: &[IncompleteLine]) -> usize {
    // We need to sort all the scores, and then select the middle score.
    let mut scores: Vec<usize> = incomplete_lines.iter().map(|x| x.score()).collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
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
    fn test_part1_syntax_errors() {
        let score = score_for(&syntax_errors_in(INPUT));
        assert_eq!(score, 26397);
    }

    #[test]
    fn test_part2_incomplete_lines() {
        let completed_expected = "\
            [({(<(())[]>[[{[]{<()<>>}}]])})]
            [(()[<>])]({[<{<<[]>>()}>]})
            (((({<>}<{<{<>}{[]{[]{}}}>}>))))
            {<[[]]>}<{[{[{[]{()[[[]]]}}]}]}>
            <{([{{}}[<[[[<>{}]]]>[]]])}>
        ";
        let completed_expected: Vec<&str> = completed_expected
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();

        let incomplete_lines = incomplete_lines_in(INPUT);
        let completed_actual: Vec<String> =
            incomplete_lines.iter().map(|x| x.completed()).collect();
        assert_eq!(completed_actual, completed_expected);

        let completion_score = completion_score_of(&incomplete_lines);
        assert_eq!(completion_score, 288957);
    }
}
