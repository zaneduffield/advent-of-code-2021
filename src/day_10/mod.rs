use itertools::Itertools;

struct ParseErr {
    c: char,
}

fn parse_line(line: &str) -> Result<Vec<char>, ParseErr> {
    let mut stack = Vec::with_capacity(line.len());
    for c in line.chars() {
        if Some(&c) == stack.last() {
            stack.pop();
            continue;
        }
        stack.push(match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return Err(ParseErr { c }),
        });
    }

    Ok(stack)
}

#[aoc(day10, part1)]
pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_line)
        .flat_map(Result::err)
        .map(|ParseErr { c }| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected character {}", c),
        })
        .sum()
}

fn score_stack(stack: Vec<char>) -> u64 {
    stack
        .iter()
        .rev()
        .map(|&c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected character {}", c),
        })
        .reduce(|score, c| score * 5 + c)
        .unwrap_or(0)
}

#[aoc(day10, part2)]
pub fn part_2(input: &str) -> u64 {
    let scores: Vec<_> = input
        .lines()
        .map(parse_line)
        .flat_map(Result::ok)
        .map(score_stack)
        .sorted_unstable()
        .collect();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(26397, part_1(input));
        assert_eq!(288957, part_2(input));
    }
}
