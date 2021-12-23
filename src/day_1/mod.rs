#[aoc(day1, part1)]
pub fn part_1(contents: &str) -> usize {
    count_window_increases(contents, 1)
}

#[aoc(day1, part2)]
pub fn part_2(contents: &str) -> usize {
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
    use super::*;

    #[test]
    fn test() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(part_1(input), 7);
        assert_eq!(part_2(input), 5);
    }
}
