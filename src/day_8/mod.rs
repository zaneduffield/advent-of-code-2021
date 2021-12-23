use itertools::Itertools;

fn parse_1(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.split(" | ").last().unwrap().split_whitespace())
        .map(str::len)
        .collect()
}

#[aoc(day8, part1)]
pub fn part_1(input: &str) -> usize {
    let lens = parse_1(input);
    lens.iter().filter(|&&len| len != 5 && len != 6).count()
}

type Word = Vec<u8>;
struct Pattern {
    signals: Vec<Word>,
    outputs: Vec<Word>,
}

fn parse_2(input: &str) -> Vec<Pattern> {
    input
        .lines()
        .map(|line| {
            let (signals, outputs) = line
                .split(" | ")
                .map(|s| s.split_whitespace().map(|s| s.bytes().collect()).collect())
                .collect_tuple()
                .unwrap();
            Pattern { signals, outputs }
        })
        .collect()
}

fn vec_eq<T: Eq>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| *x == *y)
}

fn num_from_bytes(bytes: &[u8]) -> u32 {
    str::parse(std::str::from_utf8(bytes).unwrap()).unwrap()
}

fn decode(pattern: &Pattern) -> u32 {
    let mut freqs = [0; 7];
    let mut lengths_found_in = [[false; 7]; 7];
    for signal in pattern.signals.iter() {
        for c in signal.iter().map(|c| c - b'a') {
            freqs[c as usize] += 1;
            lengths_found_in[c as usize][signal.len() - 1] = true;
        }
    }

    let lengths_counts = lengths_found_in.map(|counts| counts.iter().filter(|b| **b).count());

    // these properties just happen to be enough to uniquely identify the segments
    let mut map = [b'0'; 7];
    for i in 0..freqs.len() {
        let old_digit = b'a' + i as u8;
        match (freqs[i], lengths_counts[i]) {
            (8, 4) => map[0] = old_digit,
            (6, _) => map[1] = old_digit,
            (8, 6) => map[2] = old_digit,
            (7, 4) => map[3] = old_digit,
            (4, _) => map[4] = old_digit,
            (9, _) => map[5] = old_digit,
            (7, 3) => map[6] = old_digit,
            _ => panic!("Invalid freq + count pair"),
        }
    }

    let to_word =
        |indexes: &[usize]| -> Word { indexes.iter().map(|&n| map[n]).sorted().collect() };

    let mapping: [Word; 10] = [
        to_word(&[0, 1, 2, 4, 5, 6]),
        to_word(&[2, 5]),
        to_word(&[0, 2, 3, 4, 6]),
        to_word(&[0, 2, 3, 5, 6]),
        to_word(&[1, 2, 3, 5]),
        to_word(&[0, 1, 3, 5, 6]),
        to_word(&[0, 1, 3, 4, 5, 6]),
        to_word(&[0, 2, 5]),
        to_word(&[0, 1, 2, 3, 4, 5, 6]),
        to_word(&[0, 1, 2, 3, 5, 6]),
    ];

    // we need to sort the outputs so they can be compared directly with the sorted bytes above
    let mut outputs = pattern.outputs.clone();
    for o in outputs.iter_mut() {
        o.sort_unstable()
    }

    num_from_bytes(
        &outputs
            .iter()
            .map(|o| mapping.iter().position(|num| vec_eq(num, o)).unwrap() as u8 + b'0')
            .collect::<Vec<_>>(),
    )
}

#[aoc(day8, part2)]
pub fn part_2(input: &str) -> u32 {
    let patterns = parse_2(input);
    patterns.iter().map(decode).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe 
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(26, part_1(input));
        assert_eq!(61229, part_2(input));
    }
}
