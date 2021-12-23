#[aoc(day2, part1)]
pub fn part_1(contents: &str) -> i64 {
    let (hor, depth) = contents.lines().fold((0, 0), |(hor, depth), s| {
        let (word, num) = s.split_once(" ").unwrap();
        let num: i64 = num.parse().unwrap();
        match word {
            "forward" => (hor + num, depth),
            "down" => (hor, depth + num),
            "up" => (hor, depth - num),
            _ => panic!("unexpected line: {}", s),
        }
    });

    hor * depth
}

#[aoc(day2, part2)]
pub fn part_2(contents: &str) -> i64 {
    let (hor, depth, _aim) = contents.lines().fold((0, 0, 0), |(hor, depth, aim), s| {
        let (word, num) = s.split_once(" ").unwrap();
        let num: i64 = num.parse().unwrap();
        match word {
            "forward" => (hor + num, depth + aim * num, aim),
            "down" => (hor, depth, aim + num),
            "up" => (hor, depth, aim - num),
            _ => panic!("unexpected line: {}", s),
        }
    });

    hor * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part_1(input), 150);
        assert_eq!(part_2(input), 900);
    }
}
