fn get_most_common_bit_string(contents: &str) -> String {
    contents
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| if c == '1' { 1 } else { -1 })
                .collect::<Vec<_>>()
        })
        .reduce(|v1, v2| v1.iter().zip(v2.iter()).map(|(a1, a2)| a1 + a2).collect())
        .unwrap()
        .iter()
        .map(|count| if *count >= 0 { '1' } else { '0' })
        .collect()
}

fn invert_bit_string(bits: &str) -> String {
    bits.chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect()
}

#[aoc(day3, part1)]
fn part_1(contents: &str) -> u64 {
    let most_common_bits = get_most_common_bit_string(contents);
    let gamma = u64::from_str_radix(&most_common_bits, 2).unwrap();
    let epsilon = u64::from_str_radix(&invert_bit_string(&most_common_bits), 2).unwrap();

    gamma * epsilon
}

fn reading<T>(lines: &[&str], compare: T) -> u64
where
    T: Fn(usize, usize) -> bool,
{
    let mut remaining = lines.to_vec();
    let mut i = 0;
    while remaining.len() > 1 {
        let num_matches = remaining
            .iter()
            .filter(|s| s.chars().nth(i) == Some('1'))
            .count();

        let selected = if compare(num_matches, remaining.len()) {
            '1'
        } else {
            '0'
        };

        remaining = remaining
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(selected))
            .collect();

        i += 1;
    }

    u64::from_str_radix(remaining[0], 2).unwrap()
}

#[aoc(day3, part2)]
fn part_2(contents: &str) -> u64 {
    let lines = contents.lines().collect::<Vec<_>>();
    let oxy = reading(&lines, |num_ones, len| 2 * num_ones >= len);
    let co2 = reading(&lines, |num_ones, len| 2 * num_ones < len);

    oxy * co2
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("01\n10\n01"), 2);
        assert_eq!(part_1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 230);
    }
}
