pub fn run() {
    let input = include_str!("input.txt");
    println!(
        "day 2\n  part 1: {}\n  part 2: {}\n",
        part_1(input),
        part_2(input)
    );
}

fn part_1(contents: &str) -> i64 {
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

fn part_2(contents: &str) -> i64 {
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
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(""), 0);
        assert_eq!(part_1("down 1"), 0);
        assert_eq!(part_1("down 2\nforward 5"), 10);
        assert_eq!(part_1("down 52\nup 2\nforward 10"), 500);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
        assert_eq!(part_2("down 1"), 0);
        assert_eq!(part_2("down 2\nforward 5"), 50);
        assert_eq!(part_2("down 52\nup 2\nforward 10"), 5000);
    }
}
