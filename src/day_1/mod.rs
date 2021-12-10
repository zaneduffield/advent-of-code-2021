#[aoc(day1, part1)]
fn part_1(contents: &str) -> usize {
    count_window_increases(contents, 1)
}

#[aoc(day1, part2)]
fn part_2(contents: &str) -> usize {
    count_window_increases(contents, 3)
}

fn count_window_increases(contents: &str, window_size: usize) -> usize {
    contents
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .windows(window_size + 1)
        .filter(|w| w.last() > w.first())
        .count()
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(""), 0);
        assert_eq!(part_1("1"), 0);
        assert_eq!(part_1("1\n1\n2\n-1\n3"), 2);
        assert_eq!(part_1("-1\n3\n3\n4\n10\n101\n101\n102"), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
        assert_eq!(part_2("1"), 0);
        assert_eq!(part_2("1\n1\n2\n-1\n3"), 1);
        assert_eq!(part_2("-1\n3\n3\n4\n10\n101\n101\n102"), 5);
    }
}
